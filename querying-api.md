# ORMery - Querying API Comparison

A side-by-side comparison of how each language's ORM handles common database operations. All apps use the same SQLite schema with `lists` and `todos` tables.

---

## Schema / Model Definition

| Language | Approach | Key Syntax |
|----------|----------|------------|
| **JS** | Raw DDL + prepared statements | `db.prepare('SELECT * FROM lists WHERE id = ?')` |
| **Python** | Declarative class inheriting `db.Model` | `db.Column(db.Integer, primary_key=True)` |
| **C#** | POCO class + Fluent API in `DbContext` | `entity.Property(e => e.Name).IsRequired()` |
| **Rust** | Struct with `#[derive(FromRow)]` | `struct TodoList { id: i64, name: String }` |
| **Java** | Class with `@Entity` + `@Table` annotations | `@Column(nullable = false)` |
| **Lua** | Raw DDL strings (no model classes) | `conn:exec("CREATE TABLE IF NOT EXISTS ...")` |

### Relationships

| Language | One-to-Many Declaration | Cascade Delete |
|----------|------------------------|----------------|
| **JS** | `REFERENCES lists(id)` in DDL | Manual transaction (delete todos then list) |
| **Python** | `db.relationship("Todo", cascade="all, delete-orphan")` | Via relationship config |
| **C#** | `entity.HasOne().WithMany().OnDelete(DeleteBehavior.Cascade)` | Fluent API |
| **Rust** | No ORM relationship -- manual `ON DELETE CASCADE` in DDL | DDL foreign key |
| **Java** | `@OneToMany(cascade = CascadeType.ALL, orphanRemoval = true)` | Annotation |
| **Lua** | `REFERENCES lists(id) ON DELETE CASCADE` in DDL | DDL foreign key |

---

## SELECT All

Fetch all lists, ordered by creation date.

**JS (better-sqlite3)**
```javascript
const lists = db.prepare(`
  SELECT l.*, (SELECT COUNT(*) FROM todos WHERE list_id = l.id) AS todo_count,
    (SELECT COUNT(*) FROM todos WHERE list_id = l.id AND completed = 1) AS done_count
  FROM lists l ORDER BY l.created_at DESC
`).all();
```

**Python (SQLAlchemy)**
```python
lists = List.query.order_by(List.created_at.asc()).all()
```

**C# (EF Core)**
```csharp
var lists = await _db.Lists
    .Include(l => l.Todos)
    .OrderBy(l => l.CreatedAt)
    .ToListAsync();
```

**Rust (SQLx)**
```rust
let rows: Vec<(i64, String, i64)> = sqlx::query_as(
    "SELECT l.id, l.name, COUNT(t.id) as todo_count
     FROM lists l LEFT JOIN todos t ON t.list_id = l.id
     GROUP BY l.id ORDER BY l.created_at DESC"
).fetch_all(&pool).await?;
```

**Java (Spring Data JPA)**
```java
List<TodoList> lists = listRepository.findAll();
```

**Lua (lsqlite3)**
```lua
local stmt = conn:prepare("SELECT l.id, l.name, COUNT(t.id) ... GROUP BY l.id")
while stmt:step() == sqlite3.ROW do
    table.insert(results, { id = stmt:get_value(0), name = stmt:get_value(1) })
end
stmt:finalize()
```

---

## SELECT by ID

Fetch a single list by primary key.

**JS**
```javascript
const list = db.prepare('SELECT * FROM lists WHERE id = ?').get(id);
```

**Python**
```python
lst = db.get_or_404(List, list_id)
```

**C#**
```csharp
var list = await _db.Lists.FindAsync(id);
```

**Rust**
```rust
let list: TodoList = sqlx::query_as("SELECT id, name, created_at FROM lists WHERE id = ?")
    .bind(id).fetch_one(&pool).await?;
```

**Java**
```java
TodoList list = listRepository.findById(id).orElse(null);
```

**Lua**
```lua
local stmt = conn:prepare("SELECT id, name, created_at FROM lists WHERE id = ?")
stmt:bind_values(id)
if stmt:step() == sqlite3.ROW then
    result = { id = stmt:get_value(0), name = stmt:get_value(1) }
end
stmt:finalize()
```

---

## SELECT with JOINs / Filtering

Fetch todos for a specific list, with counts.

**JS**
```javascript
const todos = db.prepare(
  'SELECT * FROM todos WHERE list_id = ? ORDER BY completed ASC, created_at DESC'
).all(id);
const counts = db.prepare(
  'SELECT COUNT(*) AS total, SUM(completed) AS done FROM todos WHERE list_id = ?'
).get(id);
```

**Python**
```python
todos = Todo.query.filter_by(list_id=lst.id).order_by(Todo.created_at.asc()).all()
done = Todo.query.filter_by(list_id=lst.id, completed=True).count()
```

**C#**
```csharp
// Eager-loaded via Include -- access via navigation property
var todos = list.Todos;
var completedCount = list.Todos.Count(t => t.Completed);
```

**Rust**
```rust
let todos: Vec<TodoItem> = sqlx::query_as(
    "SELECT id, title, completed, list_id, created_at
     FROM todos WHERE list_id = ? ORDER BY completed ASC, created_at DESC"
).bind(id).fetch_all(&pool).await?;
let completed_count = todos.iter().filter(|t| t.completed).count();
```

**Java**
```java
// Derived query method -- Spring generates SQL from method name
List<TodoItem> todos = itemRepository.findByListIdOrderByCreatedAtAsc(listId);
long completed = list.getTodos().stream().filter(TodoItem::getCompleted).count();
```

**Lua**
```lua
local stmt = conn:prepare([[
    SELECT id, title, completed, list_id, created_at
    FROM todos WHERE list_id = ? ORDER BY completed ASC, created_at DESC
]])
stmt:bind_values(list_id)
while stmt:step() == sqlite3.ROW do
    table.insert(results, {
        id = stmt:get_value(0), title = stmt:get_value(1),
        completed = stmt:get_value(2)
    })
end
stmt:finalize()
```

---

## INSERT

Create a new list / todo.

**JS**
```javascript
db.prepare('INSERT INTO lists (name) VALUES (?)').run(name);
```

**Python**
```python
new_list = List(name="Work")
db.session.add(new_list)
db.session.commit()
```

**C#**
```csharp
_db.Lists.Add(new TodoList { Name = name.Trim(), CreatedAt = DateTime.UtcNow });
await _db.SaveChangesAsync();
```

**Rust**
```rust
sqlx::query("INSERT INTO lists (name) VALUES (?)")
    .bind(&name)
    .execute(&pool)
    .await?;
```

**Java**
```java
listRepository.save(new TodoList(name.trim()));
```

**Lua**
```lua
local stmt = conn:prepare("INSERT INTO lists (name) VALUES (?)")
stmt:bind_values(name)
stmt:step()
stmt:finalize()
return conn:last_insert_rowid()
```

---

## UPDATE

Toggle a todo's completed status.

**JS**
```javascript
db.prepare(
  'UPDATE todos SET completed = CASE WHEN completed = 1 THEN 0 ELSE 1 END WHERE id = ?'
).run(id);
```

**Python**
```python
todo.completed = not todo.completed
db.session.commit()
```

**C#**
```csharp
todo.Completed = !todo.Completed;
await _db.SaveChangesAsync();
```

**Rust**
```rust
sqlx::query("UPDATE todos SET completed = ? WHERE id = ?")
    .bind(!completed)
    .bind(id)
    .execute(&pool)
    .await?;
```

**Java**
```java
item.setCompleted(!item.getCompleted());
itemRepository.save(item);
```

**Lua**
```lua
-- Clever arithmetic toggle: 1 - 0 = 1, 1 - 1 = 0
local stmt = conn:prepare("UPDATE todos SET completed = 1 - completed WHERE id = ?")
stmt:bind_values(id)
stmt:step()
stmt:finalize()
```

---

## DELETE with Cascades

Delete a list and all its todos.

**JS**
```javascript
// Manual cascade via transaction
const deleteListAndTodos = db.transaction((listId) => {
  db.prepare('DELETE FROM todos WHERE list_id = ?').run(listId);
  db.prepare('DELETE FROM lists WHERE id = ?').run(listId);
});
deleteListAndTodos(id);
```

**Python**
```python
# Cascade configured in relationship: cascade="all, delete-orphan"
db.session.delete(lst)
db.session.commit()
```

**C#**
```csharp
// Cascade configured in Fluent API: .OnDelete(DeleteBehavior.Cascade)
_db.Lists.Remove(list);
await _db.SaveChangesAsync();
```

**Rust**
```rust
// Manual cascade (FK cascade may not be enforced)
sqlx::query("DELETE FROM todos WHERE list_id = ?").bind(id).execute(&pool).await?;
sqlx::query("DELETE FROM lists WHERE id = ?").bind(id).execute(&pool).await?;
```

**Java**
```java
// Cascade configured via @OneToMany(cascade = CascadeType.ALL, orphanRemoval = true)
listRepository.deleteById(id);
```

**Lua**
```lua
-- FK cascade configured in DDL: ON DELETE CASCADE
-- Plus PRAGMA foreign_keys=ON at connection time
local stmt = conn:prepare("DELETE FROM lists WHERE id = ?")
stmt:bind_values(id)
stmt:step()
stmt:finalize()
```

---

## Ordering and Filtering

| Language | Order By | Filter By |
|----------|----------|-----------|
| **JS** | `ORDER BY completed ASC, created_at DESC` (SQL) | `WHERE list_id = ?` (SQL) |
| **Python** | `.order_by(Todo.created_at.asc())` | `.filter_by(list_id=id, completed=True)` |
| **C#** | `.OrderBy(l => l.CreatedAt)` | `.Where(t => t.ListId == id)` or LINQ |
| **Rust** | `ORDER BY completed ASC, created_at DESC` (SQL) | `WHERE list_id = ?` (SQL) |
| **Java** | `@OrderBy("createdAt ASC")` or method name `OrderByCreatedAtAsc` | Method name `findByListId...` |
| **Lua** | `ORDER BY completed ASC, created_at DESC` (SQL) | `WHERE list_id = ?` (SQL) |

---

## Summary

| | Abstraction Level | Query Style | Type Safety | Relationship Mgmt |
|-|-------------------|-------------|-------------|-------------------|
| **JS** | None (raw SQL) | Sync prepared statements | None | Manual transaction |
| **Python** | High | Fluent ORM API | Runtime | ORM relationship() |
| **C#** | High | LINQ expressions | Compile-time | Fluent API navigation |
| **Rust** | Low (SQL mapper) | Raw SQL + bind | Compile-time (FromRow) | Manual / DDL |
| **Java** | High | Derived query methods | Compile-time | JPA annotations |
| **Lua** | None (raw SQL) | Prepared statements | None | DDL foreign keys |

The spectrum runs from **Java** (most abstract -- just name a method and Spring generates the query) through **Python/C#** (fluent ORM APIs) to **Rust** (type-safe SQL mapper) to **JS/Lua** (raw prepared statements with manual row mapping). JS uses synchronous `better-sqlite3` while Lua uses the `lsqlite3` C binding -- both are thin wrappers over SQLite's C API.
