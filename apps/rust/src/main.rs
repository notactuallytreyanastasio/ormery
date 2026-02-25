use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use ormery::{field, schema, to_insert_sql, Field, InMemoryStore, Query, Schema};
use rusqlite::Connection;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tower_http::services::ServeDir;

// ---------------------------------------------------------------------------
// App state
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
    list_schema: Schema,
    todo_schema: Schema,
    /// Dummy store required by ORMery Query constructor (queries use to_sql,
    /// not the in-memory engine).
    store: InMemoryStore,
}

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct TodoList {
    id: i64,
    name: String,
    #[allow(dead_code)]
    created_at: Option<String>,
}

#[derive(Debug)]
struct TodoItem {
    id: i64,
    title: String,
    completed: bool,
    #[allow(dead_code)]
    list_id: i64,
    #[allow(dead_code)]
    created_at: Option<String>,
}

/// Extended list info with todo count for index page
struct ListWithCount {
    id: i64,
    name: String,
    todo_count: i64,
}

// ---------------------------------------------------------------------------
// Templates
// ---------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    lists: Vec<ListWithCount>,
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
    list: TodoList,
    todos: Vec<TodoItem>,
    completed_count: usize,
}

// ---------------------------------------------------------------------------
// Form types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct CreateListForm {
    name: String,
}

#[derive(Deserialize)]
struct CreateTodoForm {
    title: String,
}

#[derive(Deserialize)]
struct EditTodoForm {
    title: String,
}

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

struct AppError(Box<dyn std::error::Error + Send + Sync>);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// ---------------------------------------------------------------------------
// ORMery schema definitions
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Database setup
// ---------------------------------------------------------------------------

fn init_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Enable WAL mode and foreign keys
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

    // Create tables
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS lists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed INTEGER DEFAULT 0,
            list_id INTEGER REFERENCES lists(id) ON DELETE CASCADE,
            created_at TEXT DEFAULT (datetime('now'))
        );",
    )?;

    // Seed data if empty
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM lists", [], |row| row.get(0))?;

    if count == 0 {
        conn.execute("INSERT INTO lists (name) VALUES (?1)", ["Personal"])?;
        conn.execute("INSERT INTO lists (name) VALUES (?1)", ["Work"])?;

        for title in &[
            "Buy groceries",
            "Clean the house",
            "Call mom",
            "Read a book",
        ] {
            conn.execute(
                "INSERT INTO todos (title, list_id) VALUES (?1, 1)",
                [title],
            )?;
        }
        conn.execute(
            "UPDATE todos SET completed = 1 WHERE title = 'Buy groceries'",
            [],
        )?;

        for title in &[
            "Finish quarterly report",
            "Review pull requests",
            "Update documentation",
            "Team standup notes",
            "Deploy to staging",
        ] {
            conn.execute(
                "INSERT INTO todos (title, list_id) VALUES (?1, 2)",
                [title],
            )?;
        }
        conn.execute(
            "UPDATE todos SET completed = 1 WHERE title = 'Review pull requests'",
            [],
        )?;
        conn.execute(
            "UPDATE todos SET completed = 1 WHERE title = 'Team standup notes'",
            [],
        )?;

        println!("Seeded database with sample data.");
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build an ORMery SELECT query for todos filtered by list_id.
fn todos_for_list_sql(state: &AppState, list_id: i64) -> String {
    let frag = Query::new(state.todo_schema.clone(), state.store.clone())
        .r#where("list_id", "=", list_id.to_string())
        .order_by("completed", "asc")
        .to_sql();
    frag.to_string().to_string()
}

/// Build an ORMery INSERT statement for the lists table.
fn insert_list_sql(state: &AppState, name: &str) -> String {
    let values = temper_core::Map::new(&[
        (Arc::new("name".to_string()), Arc::new(name.to_string())),
    ]);
    to_insert_sql(state.list_schema.clone(), values)
        .to_string()
        .to_string()
}

/// Build an ORMery INSERT statement for the todos table.
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

/// Build an ORMery SELECT for a single list by id.
fn list_by_id_sql(state: &AppState, id: i64) -> String {
    let frag = Query::new(state.list_schema.clone(), state.store.clone())
        .r#where("id", "=", id.to_string())
        .limit(1)
        .to_sql();
    frag.to_string().to_string()
}

/// Build an ORMery SELECT for a single todo by id.
fn todo_by_id_sql(state: &AppState, id: i64) -> String {
    let frag = Query::new(state.todo_schema.clone(), state.store.clone())
        .r#where("id", "=", id.to_string())
        .limit(1)
        .to_sql();
    frag.to_string().to_string()
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// GET / - Show all lists
async fn index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let lists = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();
            // Use raw SQL for the JOIN + aggregate (ORMery doesn't support joins)
            let mut stmt = conn
                .prepare(
                    "SELECT l.id, l.name, COUNT(t.id) as todo_count
                     FROM lists l
                     LEFT JOIN todos t ON t.list_id = l.id
                     GROUP BY l.id
                     ORDER BY l.created_at DESC",
                )
                .unwrap();
            let rows = stmt
                .query_map([], |row| {
                    Ok(ListWithCount {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        todo_count: row.get(2)?,
                    })
                })
                .unwrap();
            rows.collect::<Result<Vec<_>, _>>().unwrap()
        })
        .await?
    };

    let template = IndexTemplate { lists };
    Ok(Html(template.render()?))
}

/// POST /lists - Create a new list (uses ORMery to_insert_sql)
async fn create_list(
    State(state): State<AppState>,
    Form(form): Form<CreateListForm>,
) -> Result<Redirect, AppError> {
    let name = form.name.trim().to_string();
    if !name.is_empty() {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let sql = insert_list_sql(&state, &name);
            let conn = state.db.lock().unwrap();
            conn.execute(&sql, []).unwrap();
        })
        .await?;
    }
    Ok(Redirect::to("/"))
}

/// POST /lists/:id/delete - Delete a list
async fn delete_list(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let state = state.clone();
    tokio::task::spawn_blocking(move || {
        let conn = state.db.lock().unwrap();
        // ORMery doesn't generate DELETE; use raw SQL
        conn.execute("DELETE FROM todos WHERE list_id = ?1", [id])
            .unwrap();
        conn.execute("DELETE FROM lists WHERE id = ?1", [id])
            .unwrap();
    })
    .await?;
    Ok(Redirect::to("/"))
}

/// GET /lists/:id - Show a list with its todos (uses ORMery Query for SELECT)
async fn show_list(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let (list, todos) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            // Fetch list using ORMery-generated SELECT
            let list_sql = list_by_id_sql(&state, id);
            let list = conn
                .query_row(&list_sql, [], |row| {
                    Ok(TodoList {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        created_at: row.get(2)?,
                    })
                })
                .unwrap();

            // Fetch todos using ORMery-generated SELECT
            let todos_sql = todos_for_list_sql(&state, id);
            let mut stmt = conn.prepare(&todos_sql).unwrap();
            let rows = stmt
                .query_map([], |row| {
                    Ok(TodoItem {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        completed: {
                            let v: i64 = row.get(2)?;
                            v != 0
                        },
                        list_id: row.get(3)?,
                        created_at: row.get(4)?,
                    })
                })
                .unwrap();
            let todos: Vec<TodoItem> = rows.collect::<Result<Vec<_>, _>>().unwrap();

            (list, todos)
        })
        .await?
    };

    let completed_count = todos.iter().filter(|t| t.completed).count();

    let template = ListTemplate {
        list,
        todos,
        completed_count,
    };
    Ok(Html(template.render()?))
}

/// POST /lists/:id/todos - Create a todo in a list (uses ORMery to_insert_sql)
async fn create_todo(
    State(state): State<AppState>,
    Path(list_id): Path<i64>,
    Form(form): Form<CreateTodoForm>,
) -> Result<Redirect, AppError> {
    let title = form.title.trim().to_string();
    if !title.is_empty() {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let sql = insert_todo_sql(&state, &title, list_id);
            let conn = state.db.lock().unwrap();
            conn.execute(&sql, []).unwrap();
        })
        .await?;
    }
    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

/// POST /todos/:id/toggle - Toggle todo completed status
async fn toggle_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let list_id = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            // Fetch current todo using ORMery-generated SELECT
            let sql = todo_by_id_sql(&state, id);
            let (completed, list_id): (bool, i64) = conn
                .query_row(&sql, [], |row| {
                    let c: i64 = row.get(2)?;
                    Ok((c != 0, row.get(3)?))
                })
                .unwrap();

            // UPDATE is not supported by ORMery; use raw SQL
            conn.execute(
                "UPDATE todos SET completed = ?1 WHERE id = ?2",
                rusqlite::params![!completed as i64, id],
            )
            .unwrap();

            list_id
        })
        .await?
    };

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

/// POST /todos/:id/delete - Delete a todo
async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let list_id = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            // Fetch list_id using ORMery-generated SELECT
            let sql = todo_by_id_sql(&state, id);
            let list_id: i64 = conn.query_row(&sql, [], |row| row.get(3)).unwrap();

            // DELETE is not supported by ORMery; use raw SQL
            conn.execute("DELETE FROM todos WHERE id = ?1", [id])
                .unwrap();

            list_id
        })
        .await?
    };

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

/// POST /todos/:id/edit - Edit a todo title
async fn edit_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<EditTodoForm>,
) -> Result<Redirect, AppError> {
    let list_id = {
        let state = state.clone();
        let title = form.title.trim().to_string();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            // Fetch list_id using ORMery-generated SELECT
            let sql = todo_by_id_sql(&state, id);
            let list_id: i64 = conn.query_row(&sql, [], |row| row.get(3)).unwrap();

            // UPDATE is not supported by ORMery; use raw SQL
            if !title.is_empty() {
                conn.execute(
                    "UPDATE todos SET title = ?1 WHERE id = ?2",
                    rusqlite::params![title, id],
                )
                .unwrap();
            }

            list_id
        })
        .await?
    };

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize ORMery runtime (required for Temper-compiled code)
    ormery::init(None).unwrap();

    // Open rusqlite connection
    let conn = Connection::open("todo.db")?;
    init_db(&conn)?;

    // Build ORMery schemas
    let list_schema = build_list_schema();
    let todo_schema = build_todo_schema();
    let store = InMemoryStore::new();

    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
        list_schema,
        todo_schema,
        store,
    };

    // Build routes
    let app = Router::new()
        .route("/", get(index))
        .route("/lists", post(create_list))
        .route("/lists/{id}", get(show_list))
        .route("/lists/{id}/delete", post(delete_list))
        .route("/lists/{id}/todos", post(create_todo))
        .route("/todos/{id}/toggle", post(toggle_todo))
        .route("/todos/{id}/delete", post(delete_todo))
        .route("/todos/{id}/edit", post(edit_todo))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    println!("Todo App running at http://localhost:5003");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5003").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
