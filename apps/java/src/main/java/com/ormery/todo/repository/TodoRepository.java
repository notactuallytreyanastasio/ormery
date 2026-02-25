package com.ormery.todo.repository;

import com.ormery.todo.model.TodoItem;
import com.ormery.todo.model.TodoList;
import ormery.Field;
import ormery.InMemoryStore;
import ormery.OrmeryGlobal;
import ormery.Query;
import ormery.Schema;
import ormery.sql.SqlFragment;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.core.RowMapper;
import org.springframework.stereotype.Repository;

import jakarta.annotation.PostConstruct;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/**
 * JDBC repository backed by ORMery for query building.
 *
 * ORMery handles: schema definition, SELECT generation, INSERT generation.
 * Raw JDBC handles: DDL, UPDATE, DELETE, and executing the generated SQL.
 */
@Repository
public class TodoRepository {

    private final JdbcTemplate jdbc;

    // ORMery schemas
    private final Schema listSchema;
    private final Schema itemSchema;

    // Dummy store required by Query constructor (not used for actual storage)
    private final InMemoryStore dummyStore = new InMemoryStore();

    private static final DateTimeFormatter DT_FMT = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");

    public TodoRepository(JdbcTemplate jdbc) {
        this.jdbc = jdbc;

        // Define schemas using ORMery
        this.listSchema = OrmeryGlobal.schema("lists", List.of(
            OrmeryGlobal.field("name", "String", false, false),
            OrmeryGlobal.field("created_at", "String", false, false)
        ));

        this.itemSchema = OrmeryGlobal.schema("todos", List.of(
            OrmeryGlobal.field("title", "String", false, false),
            OrmeryGlobal.field("completed", "Int", false, false),
            OrmeryGlobal.field("created_at", "String", false, false),
            OrmeryGlobal.field("list_id", "Int", false, false)
        ));
    }

    @PostConstruct
    public void initTables() {
        jdbc.execute(
            "CREATE TABLE IF NOT EXISTS lists (" +
            "  id INTEGER PRIMARY KEY AUTOINCREMENT," +
            "  name TEXT NOT NULL," +
            "  created_at TEXT NOT NULL" +
            ")"
        );
        jdbc.execute(
            "CREATE TABLE IF NOT EXISTS todos (" +
            "  id INTEGER PRIMARY KEY AUTOINCREMENT," +
            "  title TEXT NOT NULL," +
            "  completed INTEGER NOT NULL DEFAULT 0," +
            "  created_at TEXT NOT NULL," +
            "  list_id INTEGER NOT NULL REFERENCES lists(id) ON DELETE CASCADE" +
            ")"
        );
        // Enable foreign key enforcement for SQLite
        jdbc.execute("PRAGMA foreign_keys = ON");
    }

    // ---------------------------------------------------------------
    // Row mappers
    // ---------------------------------------------------------------

    private final RowMapper<TodoList> listMapper = (rs, rowNum) -> {
        TodoList list = new TodoList();
        list.setId(rs.getLong("id"));
        list.setName(rs.getString("name"));
        list.setCreatedAt(LocalDateTime.parse(rs.getString("created_at"), DT_FMT));
        return list;
    };

    private final RowMapper<TodoItem> itemMapper = (rs, rowNum) -> {
        TodoItem item = new TodoItem();
        item.setId(rs.getLong("id"));
        item.setTitle(rs.getString("title"));
        item.setCompleted(rs.getInt("completed") != 0);
        item.setCreatedAt(LocalDateTime.parse(rs.getString("created_at"), DT_FMT));
        item.setListId(rs.getLong("list_id"));
        return item;
    };

    // ---------------------------------------------------------------
    // List operations
    // ---------------------------------------------------------------

    /**
     * Fetch all lists, with completed/total counts populated.
     * Uses ORMery to build the SELECT query for the lists table.
     */
    public List<TodoList> findAllLists() {
        // Build SELECT via ORMery
        String sql = new Query(listSchema, dummyStore)
                .orderBy("id", "asc")
                .toSql()
                .toString();

        List<TodoList> lists = jdbc.query(sql, listMapper);

        // Populate counts for each list
        for (TodoList list : lists) {
            populateListCounts(list);
        }
        return lists;
    }

    /**
     * Find a single list by id.
     * Uses ORMery to build the SELECT ... WHERE id = ? query.
     */
    public TodoList findListById(Long id) {
        String sql = new Query(listSchema, dummyStore)
                .where("id", "=", String.valueOf(id))
                .limit(1)
                .toSql()
                .toString();

        List<TodoList> results = jdbc.query(sql, listMapper);
        if (results.isEmpty()) {
            return null;
        }
        TodoList list = results.get(0);
        populateListCounts(list);
        return list;
    }

    /**
     * Insert a new list.
     * Uses ORMery to build the INSERT statement.
     */
    public void insertList(String name) {
        Map<String, String> values = new LinkedHashMap<>();
        values.put("name", name);
        values.put("created_at", LocalDateTime.now().format(DT_FMT));

        SqlFragment insertSql = OrmeryGlobal.toInsertSql(listSchema, values);
        jdbc.execute(insertSql.toString());
    }

    /**
     * Delete a list and its todos.
     * Uses raw JDBC (ORMery does not support DELETE).
     */
    public void deleteList(Long id) {
        jdbc.update("DELETE FROM todos WHERE list_id = ?", id);
        jdbc.update("DELETE FROM lists WHERE id = ?", id);
    }

    /**
     * Count all lists (used by DataLoader to check if seeding is needed).
     */
    public long countLists() {
        Long count = jdbc.queryForObject("SELECT COUNT(*) FROM lists", Long.class);
        return count != null ? count : 0;
    }

    // ---------------------------------------------------------------
    // Todo item operations
    // ---------------------------------------------------------------

    /**
     * Find all todos for a given list, ordered by created_at ascending.
     * Uses ORMery to build the SELECT query with WHERE and ORDER BY.
     */
    public List<TodoItem> findItemsByListId(Long listId) {
        String sql = new Query(itemSchema, dummyStore)
                .where("list_id", "=", String.valueOf(listId))
                .orderBy("created_at", "asc")
                .toSql()
                .toString();

        return jdbc.query(sql, itemMapper);
    }

    /**
     * Find a single todo item by id.
     * Uses ORMery to build the SELECT ... WHERE id = ? query.
     */
    public TodoItem findItemById(Long id) {
        String sql = new Query(itemSchema, dummyStore)
                .where("id", "=", String.valueOf(id))
                .limit(1)
                .toSql()
                .toString();

        List<TodoItem> results = jdbc.query(sql, itemMapper);
        return results.isEmpty() ? null : results.get(0);
    }

    /**
     * Insert a new todo item.
     * Uses ORMery to build the INSERT statement.
     */
    public void insertItem(String title, Long listId) {
        Map<String, String> values = new LinkedHashMap<>();
        values.put("title", title);
        values.put("completed", "0");
        values.put("created_at", LocalDateTime.now().format(DT_FMT));
        values.put("list_id", String.valueOf(listId));

        SqlFragment insertSql = OrmeryGlobal.toInsertSql(itemSchema, values);
        jdbc.execute(insertSql.toString());
    }

    /**
     * Insert a todo item with a specific completed state (used by DataLoader).
     * Uses ORMery to build the INSERT statement.
     */
    public void insertItem(String title, Long listId, boolean completed) {
        Map<String, String> values = new LinkedHashMap<>();
        values.put("title", title);
        values.put("completed", completed ? "1" : "0");
        values.put("created_at", LocalDateTime.now().format(DT_FMT));
        values.put("list_id", String.valueOf(listId));

        SqlFragment insertSql = OrmeryGlobal.toInsertSql(itemSchema, values);
        jdbc.execute(insertSql.toString());
    }

    /**
     * Toggle the completed state of a todo item.
     * Uses raw JDBC (ORMery does not support UPDATE).
     */
    public void toggleItem(Long id) {
        jdbc.update("UPDATE todos SET completed = CASE WHEN completed = 0 THEN 1 ELSE 0 END WHERE id = ?", id);
    }

    /**
     * Update the title of a todo item.
     * Uses raw JDBC (ORMery does not support UPDATE).
     */
    public void updateItemTitle(Long id, String title) {
        jdbc.update("UPDATE todos SET title = ? WHERE id = ?", title, id);
    }

    /**
     * Delete a todo item.
     * Uses raw JDBC (ORMery does not support DELETE).
     */
    public void deleteItem(Long id) {
        jdbc.update("DELETE FROM todos WHERE id = ?", id);
    }

    // ---------------------------------------------------------------
    // Helpers
    // ---------------------------------------------------------------

    private void populateListCounts(TodoList list) {
        Long total = jdbc.queryForObject(
            "SELECT COUNT(*) FROM todos WHERE list_id = ?", Long.class, list.getId());
        Long completed = jdbc.queryForObject(
            "SELECT COUNT(*) FROM todos WHERE list_id = ? AND completed = 1", Long.class, list.getId());
        list.setTotalCount(total != null ? total : 0);
        list.setCompletedCount(completed != null ? completed : 0);
    }
}
