# ORMery Java Demo — Spring Boot + JDBC

A retro-styled todo list app built with Spring Boot, Thymeleaf, and Spring JDBC, using the **Temper-compiled ORMery query builder** for schema definition, SELECT queries, and INSERT operations.

Port: **5004**

## How ORMery Is Vendored

The compiled ORMery library lives in `vendor/` with three subdirectories:

```
vendor/
  ormery/          ← the query builder (has src/main/java/)
  std/             ← Temper standard library
  temper-core/     ← Temper runtime
```

The ORMery Java source is added as an additional source directory via `build-helper-maven-plugin` in `pom.xml`:

```xml
<plugin>
  <groupId>org.codehaus.mojo</groupId>
  <artifactId>build-helper-maven-plugin</artifactId>
  <executions>
    <execution>
      <id>add-ormery-source</id>
      <phase>generate-sources</phase>
      <goals><goal>add-source</goal></goals>
      <configuration>
        <sources>
          <source>${project.basedir}/vendor/ormery/src/main/java</source>
        </sources>
      </configuration>
    </execution>
  </executions>
</plugin>
```

The Temper runtime is pulled from Maven Central:

```xml
<dependency>
  <groupId>dev.temperlang</groupId>
  <artifactId>temper-core</artifactId>
  <version>0.6.0</version>
</dependency>
```

No JPA, no Hibernate — just Spring JDBC (`JdbcTemplate`) for execution.

## How ORMery Is Used

### Schema Definition

From `TodoRepository.java` — schemas are built in the constructor using `OrmeryGlobal.schema()` and `OrmeryGlobal.field()`.

```java
import ormery.OrmeryGlobal;
import ormery.Query;
import ormery.Schema;
import ormery.InMemoryStore;

private final Schema listSchema;
private final Schema itemSchema;
private final InMemoryStore dummyStore = new InMemoryStore();

public TodoRepository(JdbcTemplate jdbc) {
    this.jdbc = jdbc;

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
```

### SELECT Queries (ORMery)

ORMery builds the SQL string, JdbcTemplate executes it with a RowMapper:

```java
// findAllLists — SELECT * FROM lists ORDER BY id ASC
public List<TodoList> findAllLists() {
    String sql = new Query(listSchema, dummyStore)
            .orderBy("id", "asc")
            .toSql()
            .toString();
    List<TodoList> lists = jdbc.query(sql, listMapper);
    for (TodoList list : lists) {
        populateListCounts(list);
    }
    return lists;
}

// findListById — SELECT with WHERE + LIMIT
public TodoList findListById(Long id) {
    String sql = new Query(listSchema, dummyStore)
            .where("id", "=", String.valueOf(id))
            .limit(1)
            .toSql()
            .toString();
    List<TodoList> results = jdbc.query(sql, listMapper);
    return results.isEmpty() ? null : results.get(0);
}

// findItemsByListId — SELECT with WHERE + ORDER BY
public List<TodoItem> findItemsByListId(Long listId) {
    String sql = new Query(itemSchema, dummyStore)
            .where("list_id", "=", String.valueOf(listId))
            .orderBy("created_at", "asc")
            .toSql()
            .toString();
    return jdbc.query(sql, itemMapper);
}
```

### INSERT Operations (ORMery)

Values are passed as a `Map<String, String>` to `OrmeryGlobal.toInsertSql()`:

```java
// insertList — create a new list
public void insertList(String name) {
    Map<String, String> values = new LinkedHashMap<>();
    values.put("name", name);
    values.put("created_at", LocalDateTime.now().format(DT_FMT));

    SqlFragment insertSql = OrmeryGlobal.toInsertSql(listSchema, values);
    jdbc.execute(insertSql.toString());
}

// insertItem — create a new todo
public void insertItem(String title, Long listId) {
    Map<String, String> values = new LinkedHashMap<>();
    values.put("title", title);
    values.put("completed", "0");
    values.put("created_at", LocalDateTime.now().format(DT_FMT));
    values.put("list_id", String.valueOf(listId));

    SqlFragment insertSql = OrmeryGlobal.toInsertSql(itemSchema, values);
    jdbc.execute(insertSql.toString());
}
```

### Raw SQL (not supported by ORMery)

UPDATE, DELETE, and aggregates use parameterized JdbcTemplate calls:

```java
// Toggle completed (ORMery doesn't generate UPDATE)
jdbc.update("UPDATE todos SET completed = CASE WHEN completed = 0 "
    + "THEN 1 ELSE 0 END WHERE id = ?", id);

// Delete a list (ORMery doesn't generate DELETE)
jdbc.update("DELETE FROM todos WHERE list_id = ?", id);
jdbc.update("DELETE FROM lists WHERE id = ?", id);

// Aggregate count (ORMery doesn't support COUNT)
Long count = jdbc.queryForObject(
    "SELECT COUNT(*) FROM todos WHERE list_id = ?", Long.class, listId);
```

## Running

```bash
./mvnw spring-boot:run
# → Todo app running at http://localhost:5004
```
