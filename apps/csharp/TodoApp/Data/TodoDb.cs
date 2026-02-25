using Microsoft.Data.Sqlite;
using Ormery;
using Ormery.Sql;
using TodoApp.Models;
using TemperLang.Core;

namespace TodoApp.Data;

/// <summary>
/// Data access layer using ORMery for query generation and Microsoft.Data.Sqlite for execution.
/// ORMery generates safe SQL via its SqlFragment type (contextual autoescaping).
/// Raw SQL is used for DDL, UPDATE, and DELETE where ORMery does not provide helpers.
/// </summary>
public class TodoDb
{
    private readonly string _connectionString;

    // ORMery schemas (table names must be lowercase -- ORMery validates identifiers)
    public static readonly Schema ListSchema = OrmeryGlobal.Schema(
        "lists",
        Listed.CreateReadOnlyList<Field>(
            OrmeryGlobal.Field("name", "String", false, false),
            OrmeryGlobal.Field("created_at", "String", false, false)
        )
    );

    public static readonly Schema TodoSchema = OrmeryGlobal.Schema(
        "todos",
        Listed.CreateReadOnlyList<Field>(
            OrmeryGlobal.Field("title", "String", false, false),
            OrmeryGlobal.Field("completed", "Int", false, false),
            OrmeryGlobal.Field("list_id", "Int", false, false),
            OrmeryGlobal.Field("created_at", "String", false, false)
        )
    );

    // Dummy store required by ORMery Query constructor (SQL generation does not use it)
    private static readonly InMemoryStore _dummyStore = new InMemoryStore();

    public TodoDb(string connectionString)
    {
        _connectionString = connectionString;
    }

    private SqliteConnection Open()
    {
        var conn = new SqliteConnection(_connectionString);
        conn.Open();
        return conn;
    }

    /// <summary>
    /// Create tables if they don't exist. Raw DDL -- ORMery does not generate DDL.
    /// </summary>
    public void EnsureCreated()
    {
        using var conn = Open();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = @"
            CREATE TABLE IF NOT EXISTS lists (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );
            CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0,
                list_id INTEGER NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (list_id) REFERENCES lists(id) ON DELETE CASCADE
            );
            PRAGMA foreign_keys = ON;
        ";
        cmd.ExecuteNonQuery();
    }

    // ────────────────────────────────────────
    // Lists
    // ────────────────────────────────────────

    /// <summary>
    /// Get all lists ordered by created_at, with their todo items attached.
    /// Uses ORMery Query.ToSql() for the SELECT.
    /// </summary>
    public List<TodoList> GetAllLists()
    {
        using var conn = Open();

        // Build SELECT * FROM lists ORDER BY created_at ASC via ORMery
        var listQuery = new Query(ListSchema, _dummyStore)
            .OrderBy("created_at", "asc");
        string listSql = listQuery.ToSql().ToString();

        var lists = new List<TodoList>();
        using (var cmd = conn.CreateCommand())
        {
            cmd.CommandText = listSql;
            using var reader = cmd.ExecuteReader();
            while (reader.Read())
            {
                lists.Add(ReadList(reader));
            }
        }

        // Attach todos to each list
        foreach (var list in lists)
        {
            list.Todos = GetTodosForList(conn, list.Id);
        }

        return lists;
    }

    /// <summary>
    /// Get a single list by id, with todos. Uses ORMery Where clause.
    /// </summary>
    public TodoList? GetList(int id)
    {
        using var conn = Open();

        var query = new Query(ListSchema, _dummyStore)
            .Where("id", "=", id.ToString());
        string sql = query.ToSql().ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        using var reader = cmd.ExecuteReader();
        if (!reader.Read())
            return null;

        var list = ReadList(reader);
        list.Todos = GetTodosForList(conn, list.Id);
        return list;
    }

    /// <summary>
    /// Insert a new list. Uses ORMery ToInsertSql.
    /// </summary>
    public void InsertList(string name)
    {
        using var conn = Open();

        var values = Mapped.ConstructMap(
            Listed.CreateReadOnlyList<KeyValuePair<string, string>>(
                new KeyValuePair<string, string>("name", name),
                new KeyValuePair<string, string>("created_at", DateTime.UtcNow.ToString("yyyy-MM-dd HH:mm:ss"))
            )
        );
        string sql = OrmeryGlobal.ToInsertSql(ListSchema, values).ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Update a list name. Raw SQL (ORMery does not provide UPDATE).
    /// Uses SqlBuilder for safe string escaping.
    /// </summary>
    public void UpdateList(int id, string name)
    {
        using var conn = Open();
        var sb = new SqlBuilder();
        sb.AppendSafe("UPDATE lists SET name = ");
        sb.AppendString(name);
        sb.AppendSafe(" WHERE id = ");
        sb.AppendInt32(id);
        string sql = sb.Accumulated.ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Delete a list and its todos. Raw SQL with SqlBuilder for safe int handling.
    /// </summary>
    public void DeleteList(int id)
    {
        using var conn = Open();

        // Enable foreign keys so CASCADE works
        using (var pragma = conn.CreateCommand())
        {
            pragma.CommandText = "PRAGMA foreign_keys = ON";
            pragma.ExecuteNonQuery();
        }

        var sb = new SqlBuilder();
        sb.AppendSafe("DELETE FROM lists WHERE id = ");
        sb.AppendInt32(id);
        string sql = sb.Accumulated.ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    // ────────────────────────────────────────
    // Todos
    // ────────────────────────────────────────

    /// <summary>
    /// Get todos for a list. Uses ORMery Query with Where and OrderBy.
    /// </summary>
    private List<TodoItem> GetTodosForList(SqliteConnection conn, int listId)
    {
        var query = new Query(TodoSchema, _dummyStore)
            .Where("list_id", "=", listId.ToString())
            .OrderBy("completed", "asc")
            .OrderBy("created_at", "asc");
        string sql = query.ToSql().ToString();

        var items = new List<TodoItem>();
        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        using var reader = cmd.ExecuteReader();
        while (reader.Read())
        {
            items.Add(ReadTodo(reader));
        }
        return items;
    }

    /// <summary>
    /// Insert a new todo. Uses ORMery ToInsertSql.
    /// </summary>
    public void InsertTodo(string title, int listId)
    {
        using var conn = Open();

        var values = Mapped.ConstructMap(
            Listed.CreateReadOnlyList<KeyValuePair<string, string>>(
                new KeyValuePair<string, string>("title", title),
                new KeyValuePair<string, string>("completed", "0"),
                new KeyValuePair<string, string>("list_id", listId.ToString()),
                new KeyValuePair<string, string>("created_at", DateTime.UtcNow.ToString("yyyy-MM-dd HH:mm:ss"))
            )
        );
        string sql = OrmeryGlobal.ToInsertSql(TodoSchema, values).ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Toggle a todo's completed status. Raw SQL with SqlBuilder.
    /// </summary>
    public void ToggleTodo(int todoId)
    {
        using var conn = Open();
        var sb = new SqlBuilder();
        sb.AppendSafe("UPDATE todos SET completed = CASE WHEN completed = 0 THEN 1 ELSE 0 END WHERE id = ");
        sb.AppendInt32(todoId);
        string sql = sb.Accumulated.ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Update a todo title. Raw SQL with SqlBuilder for safe escaping.
    /// </summary>
    public void UpdateTodo(int todoId, string title)
    {
        using var conn = Open();
        var sb = new SqlBuilder();
        sb.AppendSafe("UPDATE todos SET title = ");
        sb.AppendString(title);
        sb.AppendSafe(" WHERE id = ");
        sb.AppendInt32(todoId);
        string sql = sb.Accumulated.ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Delete a todo. Raw SQL with SqlBuilder.
    /// </summary>
    public void DeleteTodo(int todoId)
    {
        using var conn = Open();
        var sb = new SqlBuilder();
        sb.AppendSafe("DELETE FROM todos WHERE id = ");
        sb.AppendInt32(todoId);
        string sql = sb.Accumulated.ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        cmd.ExecuteNonQuery();
    }

    /// <summary>
    /// Check if any lists exist (for seeding).
    /// </summary>
    public bool HasAnyLists()
    {
        using var conn = Open();
        var query = new Query(ListSchema, _dummyStore).Limit(1);
        string sql = query.ToSql().ToString();

        using var cmd = conn.CreateCommand();
        cmd.CommandText = sql;
        using var reader = cmd.ExecuteReader();
        return reader.Read();
    }

    // ────────────────────────────────────────
    // Row mappers
    // ────────────────────────────────────────

    private static TodoList ReadList(SqliteDataReader reader)
    {
        return new TodoList
        {
            Id = reader.GetInt32(reader.GetOrdinal("id")),
            Name = reader.GetString(reader.GetOrdinal("name")),
            CreatedAt = reader.GetString(reader.GetOrdinal("created_at"))
        };
    }

    private static TodoItem ReadTodo(SqliteDataReader reader)
    {
        return new TodoItem
        {
            Id = reader.GetInt32(reader.GetOrdinal("id")),
            Title = reader.GetString(reader.GetOrdinal("title")),
            Completed = reader.GetInt32(reader.GetOrdinal("completed")) != 0,
            ListId = reader.GetInt32(reader.GetOrdinal("list_id")),
            CreatedAt = reader.GetString(reader.GetOrdinal("created_at"))
        };
    }
}
