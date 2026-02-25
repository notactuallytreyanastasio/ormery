# ORMery - Per-Language ORM Usage

Each ORMery demo app is a retro-styled todo list manager (Mac System 6 desktop + Windows 95 windows) built with a different web framework and ORM. All apps share the same schema: **lists** and **todos** with a one-to-many relationship and cascade deletes.

---

## JavaScript - Express + better-sqlite3

**Port 5006** | Express web framework with better-sqlite3 (synchronous SQLite driver) and EJS templates.

better-sqlite3 uses synchronous prepared statements. Queries are pre-compiled with `db.prepare()` and executed with `.get()` (single row), `.all()` (array), or `.run()` (write). Transactions are explicit via `db.transaction()`.

### Database Setup

```javascript
const Database = require('better-sqlite3');
const db = new Database('todo.db');
db.pragma('journal_mode = WAL');
db.pragma('foreign_keys = ON');

db.exec(`
  CREATE TABLE IF NOT EXISTS lists (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now'))
  )
`);
```

### Prepared Statements

```javascript
const stmts = {
  allLists: db.prepare(`
    SELECT l.*,
      (SELECT COUNT(*) FROM todos WHERE list_id = l.id) AS todo_count,
      (SELECT COUNT(*) FROM todos WHERE list_id = l.id AND completed = 1) AS done_count
    FROM lists l ORDER BY l.created_at DESC
  `),
  getList: db.prepare('SELECT * FROM lists WHERE id = ?'),
  createList: db.prepare('INSERT INTO lists (name) VALUES (?)'),
  toggleTodo: db.prepare(
    'UPDATE todos SET completed = CASE WHEN completed = 1 THEN 0 ELSE 1 END WHERE id = ?'
  ),
  deleteTodo: db.prepare('DELETE FROM todos WHERE id = ?'),
};
```

### Querying

```javascript
// Fetch all lists with counts (single statement, subquery approach)
const lists = stmts.allLists.all();

// Fetch a single list
const list = stmts.getList.get(id);

// Fetch todos for a list
const todos = stmts.todosByList.all(id);
```

### CRUD Operations

```javascript
// Create
stmts.createList.run(name);

// Toggle (CASE expression in SQL)
stmts.toggleTodo.run(id);

// Delete with transaction (cascade not enforced, manual cleanup)
const deleteListAndTodos = db.transaction((listId) => {
  stmts.deleteTodosByList.run(listId);
  stmts.deleteList.run(listId);
});
deleteListAndTodos(id);
```

---

## Python - Flask + SQLAlchemy

**Port 5001** | Flask web framework with SQLAlchemy as the ORM and Jinja2 templates.

SQLAlchemy uses declarative model classes. Each column is defined with `db.Column()`, and relationships are declared with `db.relationship()`. Queries use a fluent, chainable API.

### Model Definition

```python
class List(db.Model):
    __tablename__ = "lists"

    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(120), nullable=False)
    created_at = db.Column(db.DateTime, nullable=False,
                           default=lambda: datetime.now(timezone.utc))

    todos = db.relationship("Todo", backref="list", lazy=True,
                            cascade="all, delete-orphan")

class Todo(db.Model):
    __tablename__ = "todos"

    id = db.Column(db.Integer, primary_key=True)
    title = db.Column(db.String(255), nullable=False)
    completed = db.Column(db.Boolean, nullable=False, default=False)
    list_id = db.Column(db.Integer, db.ForeignKey("lists.id"), nullable=False)
    created_at = db.Column(db.DateTime, nullable=False,
                           default=lambda: datetime.now(timezone.utc))
```

### Querying

```python
# Fetch all lists ordered by creation date
lists = List.query.order_by(List.created_at.asc()).all()

# Fetch a single list (404 if missing)
lst = db.get_or_404(List, list_id)

# Fetch todos for a list
todos = Todo.query.filter_by(list_id=lst.id).order_by(Todo.created_at.asc()).all()

# Count completed todos
done = Todo.query.filter_by(list_id=lst.id, completed=True).count()
```

### CRUD Operations

```python
# Create
new_list = List(name="Work")
db.session.add(new_list)
db.session.commit()

# Update
todo.completed = not todo.completed
db.session.commit()

# Delete (cascades to todos via relationship)
db.session.delete(lst)
db.session.commit()
```

---

## C# - ASP.NET Core + Entity Framework Core

**Port 5002** | ASP.NET Core Razor Pages with EF Core as the ORM and SQLite provider.

EF Core uses POCO model classes with data annotations and a Fluent API configured in `DbContext.OnModelCreating()`. Queries use LINQ with async methods.

### Model Definition

```csharp
public class TodoList
{
    public int Id { get; set; }
    [Required] [StringLength(100)]
    public string Name { get; set; } = string.Empty;
    public DateTime CreatedAt { get; set; } = DateTime.UtcNow;
    public List<TodoItem> Todos { get; set; } = new();
}

public class TodoItem
{
    public int Id { get; set; }
    [Required] [StringLength(200)]
    public string Title { get; set; } = string.Empty;
    public bool Completed { get; set; }
    public int ListId { get; set; }
    public TodoList? List { get; set; }
    public DateTime CreatedAt { get; set; } = DateTime.UtcNow;
}
```

### DbContext (Fluent API)

```csharp
protected override void OnModelCreating(ModelBuilder modelBuilder)
{
    modelBuilder.Entity<TodoItem>(entity =>
    {
        entity.ToTable("Todos");
        entity.Property(e => e.Title).IsRequired().HasMaxLength(200);
        entity.Property(e => e.Completed).HasDefaultValue(false);
        entity.Property(e => e.CreatedAt).HasDefaultValueSql("datetime('now')");
        entity.HasOne(e => e.List)
              .WithMany(l => l.Todos)
              .HasForeignKey(e => e.ListId)
              .OnDelete(DeleteBehavior.Cascade);
    });
}
```

### Querying (LINQ)

```csharp
// Fetch all lists with their todos (eager loading)
Lists = await _db.Lists
    .Include(l => l.Todos)
    .OrderBy(l => l.CreatedAt)
    .ToListAsync();

// Find by ID
var list = await _db.Lists.FindAsync(id);

// Computed properties on model
public long GetCompletedCount() => Todos.Count(t => t.Completed);
```

### CRUD Operations

```csharp
// Create
_db.Lists.Add(new TodoList { Name = name.Trim(), CreatedAt = DateTime.UtcNow });
await _db.SaveChangesAsync();

// Update
list.Name = name.Trim();
await _db.SaveChangesAsync();

// Delete (cascade configured in Fluent API)
_db.Lists.Remove(list);
await _db.SaveChangesAsync();
```

---

## Rust - Axum + SQLx

**Port 5003** | Axum web framework with SQLx for compile-time-checked SQL queries and Askama templates.

SQLx is not a traditional ORM -- it maps SQL rows directly to Rust structs using `#[derive(FromRow)]`. Queries are raw SQL with typed parameter binding via `.bind()`.

### Model Definition

```rust
#[derive(Debug, FromRow)]
struct TodoList {
    id: i64,
    name: String,
    #[allow(dead_code)]
    created_at: Option<String>,
}

#[derive(Debug, FromRow)]
struct TodoItem {
    id: i64,
    title: String,
    completed: bool,
    #[allow(dead_code)]
    list_id: i64,
    #[allow(dead_code)]
    created_at: Option<String>,
}
```

### Querying

```rust
// Fetch all lists with counts (JOIN + GROUP BY)
let rows: Vec<(i64, String, i64)> = sqlx::query_as(
    "SELECT l.id, l.name, COUNT(t.id) as todo_count
     FROM lists l
     LEFT JOIN todos t ON t.list_id = l.id
     GROUP BY l.id
     ORDER BY l.created_at DESC",
)
.fetch_all(&pool)
.await?;

// Fetch a single list
let list: TodoList = sqlx::query_as(
    "SELECT id, name, created_at FROM lists WHERE id = ?"
)
.bind(id)
.fetch_one(&pool)
.await?;

// Fetch todos for a list
let todos: Vec<TodoItem> = sqlx::query_as(
    "SELECT id, title, completed, list_id, created_at
     FROM todos WHERE list_id = ?
     ORDER BY completed ASC, created_at DESC",
)
.bind(id)
.fetch_all(&pool)
.await?;
```

### CRUD Operations

```rust
// Create
sqlx::query("INSERT INTO lists (name) VALUES (?)")
    .bind(&name)
    .execute(&pool)
    .await?;

// Update (toggle)
sqlx::query("UPDATE todos SET completed = ? WHERE id = ?")
    .bind(!completed)
    .bind(id)
    .execute(&pool)
    .await?;

// Delete
sqlx::query("DELETE FROM todos WHERE id = ?")
    .bind(id)
    .execute(&pool)
    .await?;
```

---

## Java - Spring Boot + JPA/Hibernate

**Port 5004** | Spring Boot with Spring Data JPA (Hibernate) and Thymeleaf templates.

JPA uses `@Entity` annotations to map classes to tables. Spring Data auto-generates repository implementations from interface method names (derived queries).

### Model Definition

```java
@Entity
@Table(name = "todos")
public class TodoItem {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(nullable = false)
    private String title;

    @Column(nullable = false)
    private Boolean completed = false;

    @Column(name = "created_at", nullable = false)
    private LocalDateTime createdAt;

    @ManyToOne(fetch = FetchType.LAZY)
    @JoinColumn(name = "list_id", nullable = false)
    private TodoList list;

    @PrePersist
    protected void onCreate() {
        if (createdAt == null) createdAt = LocalDateTime.now();
        if (completed == null) completed = false;
    }
}

@Entity
@Table(name = "lists")
public class TodoList {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    @Column(nullable = false)
    private String name;

    @OneToMany(mappedBy = "list", cascade = CascadeType.ALL, orphanRemoval = true)
    @OrderBy("createdAt ASC")
    private List<TodoItem> todos = new ArrayList<>();

    public long getCompletedCount() {
        return todos.stream().filter(TodoItem::getCompleted).count();
    }
}
```

### Repository (Derived Queries)

```java
@Repository
public interface TodoItemRepository extends JpaRepository<TodoItem, Long> {
    List<TodoItem> findByListIdOrderByCreatedAtAsc(Long listId);
}
```

Spring Data JPA parses the method name `findByListIdOrderByCreatedAtAsc` and generates the SQL automatically: `SELECT * FROM todos WHERE list_id = ? ORDER BY created_at ASC`.

### CRUD Operations

```java
// Fetch all
model.addAttribute("lists", listRepository.findAll());

// Find by ID
TodoList list = listRepository.findById(id).orElse(null);

// Create
listRepository.save(new TodoList(name.trim()));

// Update (toggle)
item.setCompleted(!item.getCompleted());
itemRepository.save(item);

// Delete
itemRepository.delete(item);
```

---

## Lua - LuaSocket + lsqlite3

**Port 5005** | Hand-rolled HTTP server using LuaSocket with raw SQLite3 prepared statements via `lsqlite3complete`.

Lua has no ORM. The `db.lua` module wraps raw SQLite3 calls in named functions, using prepared statements and manual row mapping. This is the most "bare metal" approach.

### Schema (Manual DDL)

```lua
function db.migrate()
    conn:exec([[
        CREATE TABLE IF NOT EXISTS lists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now'))
        );
    ]])
    conn:exec([[
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed INTEGER DEFAULT 0,
            list_id INTEGER NOT NULL REFERENCES lists(id) ON DELETE CASCADE,
            created_at TEXT DEFAULT (datetime('now'))
        );
    ]])
end
```

### Querying

```lua
-- Fetch all lists with counts
function db.lists_get_all()
    local results = {}
    local stmt = conn:prepare([[
        SELECT l.id, l.name, l.created_at,
               COUNT(t.id) as total,
               SUM(CASE WHEN t.completed = 1 THEN 1 ELSE 0 END) as done
        FROM lists l
        LEFT JOIN todos t ON t.list_id = l.id
        GROUP BY l.id
        ORDER BY l.created_at DESC
    ]])
    while stmt:step() == sqlite3.ROW do
        table.insert(results, {
            id = stmt:get_value(0),
            name = stmt:get_value(1),
            total = stmt:get_value(3),
            done = stmt:get_value(4) or 0,
        })
    end
    stmt:finalize()
    return results
end

-- Fetch a single list by ID
function db.lists_get_by_id(id)
    local stmt = conn:prepare("SELECT id, name, created_at FROM lists WHERE id = ?")
    stmt:bind_values(id)
    local result = nil
    if stmt:step() == sqlite3.ROW then
        result = { id = stmt:get_value(0), name = stmt:get_value(1) }
    end
    stmt:finalize()
    return result
end
```

### CRUD Operations

```lua
-- Create
function db.lists_create(name)
    local stmt = conn:prepare("INSERT INTO lists (name) VALUES (?)")
    stmt:bind_values(name)
    stmt:step()
    stmt:finalize()
    return conn:last_insert_rowid()
end

-- Toggle (1 - completed flips 0/1)
function db.todos_toggle(id)
    local stmt = conn:prepare("UPDATE todos SET completed = 1 - completed WHERE id = ?")
    stmt:bind_values(id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

-- Delete
function db.todos_delete(id)
    local stmt = conn:prepare("DELETE FROM todos WHERE id = ?")
    stmt:bind_values(id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end
```
