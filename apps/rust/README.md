# ORMery Rust Demo — Axum + rusqlite

A retro-styled todo list app built with Axum, Askama templates, and rusqlite, using the **Temper-compiled ORMery query builder** for schema definition, SELECT queries, and INSERT operations.

Port: **5003**

## How ORMery Is Vendored

The compiled ORMery library lives in `vendor/` with three subdirectories:

```
vendor/
  ormery/          ← the query builder (has Cargo.toml)
  std/             ← Temper standard library
  temper-core/     ← Temper runtime (has Cargo.toml)
```

Referenced in `Cargo.toml` as path dependencies:

```toml
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
rusqlite = { version = "0.31", features = ["bundled"] }
askama = "0.13"
serde = { version = "1", features = ["derive"] }
tower-http = { version = "0.6", features = ["fs"] }
ormery = { path = "vendor/ormery" }
temper-core = { path = "vendor/temper-core" }
```

The vendored `ormery` crate internally depends on `temper-core` and `temper-std` via relative paths that resolve within `vendor/`.

No SQLx — just rusqlite (synchronous) with `tokio::task::spawn_blocking` for the async bridge.

## How ORMery Is Used

### Schema Definition

From `src/main.rs` — schemas are built at startup and stored in `AppState`. Note that Rust requires `Arc::new(fields)` for the field vector.

```rust
use ormery::{schema, field, Query, InMemoryStore, to_insert_sql, Field, Schema};
use std::sync::Arc;

fn build_list_schema() -> Schema {
    let fields: Vec<Field> = vec![
        field("name", "String", false, false),
        field("created_at", "String", false, true),
    ];
    schema("lists", Arc::new(fields))
}

fn build_todo_schema() -> Schema {
    let fields: Vec<Field> = vec![
        field("title", "String", false, false),
        field("completed", "Int", false, false),
        field("list_id", "Int", false, false),
        field("created_at", "String", false, true),
    ];
    schema("todos", Arc::new(fields))
}
```

The `ormery::init(None)` call is required at startup to initialize the Temper runtime.

### SELECT Queries (ORMery)

ORMery generates SQL via helper functions. Note `r#where` — `where` is a Rust reserved keyword:

```rust
// Fetch todos for a list, sorted by completion status
fn todos_for_list_sql(state: &AppState, list_id: i64) -> String {
    let frag = Query::new(state.todo_schema.clone(), state.store.clone())
        .r#where("list_id", "=", list_id.to_string())
        .order_by("completed", "asc")
        .to_sql();
    frag.to_string().to_string()
}

// Fetch a single list by ID
fn list_by_id_sql(state: &AppState, id: i64) -> String {
    let frag = Query::new(state.list_schema.clone(), state.store.clone())
        .r#where("id", "=", id.to_string())
        .limit(1)
        .to_sql();
    frag.to_string().to_string()
}

// Execute in a handler (async bridge via spawn_blocking)
let list_sql = list_by_id_sql(&state, id);
let list = conn.query_row(&list_sql, [], |row| {
    Ok(TodoList {
        id: row.get(0)?,
        name: row.get(1)?,
        created_at: row.get(2)?,
    })
})?;
```

### INSERT Operations (ORMery)

Values are passed as a `temper_core::Map` of `Arc<String>` key-value pairs:

```rust
// Create a new list
fn insert_list_sql(state: &AppState, name: &str) -> String {
    let values = temper_core::Map::new(&[
        (Arc::new("name".to_string()), Arc::new(name.to_string())),
    ]);
    to_insert_sql(state.list_schema.clone(), values)
        .to_string()
        .to_string()
}

// Create a new todo
fn insert_todo_sql(state: &AppState, title: &str, list_id: i64) -> String {
    let values = temper_core::Map::new(&[
        (Arc::new("title".to_string()), Arc::new(title.to_string())),
        (
            Arc::new("list_id".to_string()),
            Arc::new(list_id.to_string()),
        ),
    ]);
    to_insert_sql(state.todo_schema.clone(), values)
        .to_string()
        .to_string()
}

// Execute in a handler
let sql = insert_list_sql(&state, &name);
conn.execute(&sql, []).unwrap();
```

### Raw SQL (not supported by ORMery)

UPDATE, DELETE, and JOINs use raw rusqlite with parameterized queries:

```rust
// Toggle completed (ORMery doesn't generate UPDATE)
conn.execute(
    "UPDATE todos SET completed = ?1 WHERE id = ?2",
    rusqlite::params![!completed as i64, id],
)?;

// Delete a list (ORMery doesn't generate DELETE)
conn.execute("DELETE FROM todos WHERE list_id = ?1", [id])?;
conn.execute("DELETE FROM lists WHERE id = ?1", [id])?;

// Index page JOIN + aggregate (ORMery doesn't support JOINs)
conn.prepare(
    "SELECT l.id, l.name, COUNT(t.id) as todo_count
     FROM lists l
     LEFT JOIN todos t ON t.list_id = l.id
     GROUP BY l.id
     ORDER BY l.created_at DESC"
)?;
```

### ORMery Test Suite

The vendored ORMery crate includes 35 tests covering SQL injection prevention, query building, insert generation, and escaping:

```bash
cargo test --package ormery
# test result: ok. 35 passed; 0 failed
```

## Running

```bash
cargo run
# → Todo App running at http://localhost:5003
```
