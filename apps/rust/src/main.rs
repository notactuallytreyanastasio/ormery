use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};
use tower_http::services::ServeDir;

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

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

struct AppError(Box<dyn std::error::Error>);

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
    E: Into<Box<dyn std::error::Error>>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// ---------------------------------------------------------------------------
// Database setup
// ---------------------------------------------------------------------------

async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Enable WAL mode and foreign keys
    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(pool)
        .await?;
    sqlx::query("PRAGMA foreign_keys=ON")
        .execute(pool)
        .await?;

    // Create tables
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS lists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at TEXT DEFAULT (datetime('now'))
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed INTEGER DEFAULT 0,
            list_id INTEGER REFERENCES lists(id) ON DELETE CASCADE,
            created_at TEXT DEFAULT (datetime('now'))
        )",
    )
    .execute(pool)
    .await?;

    // Seed data if empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM lists")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        // Create "Personal" list
        sqlx::query("INSERT INTO lists (name) VALUES (?)")
            .bind("Personal")
            .execute(pool)
            .await?;
        // Create "Work" list
        sqlx::query("INSERT INTO lists (name) VALUES (?)")
            .bind("Work")
            .execute(pool)
            .await?;

        // Seed Personal todos (list_id = 1)
        for title in &[
            "Buy groceries",
            "Clean the house",
            "Call mom",
            "Read a book",
        ] {
            sqlx::query("INSERT INTO todos (title, list_id) VALUES (?, 1)")
                .bind(title)
                .execute(pool)
                .await?;
        }
        // Mark one as completed
        sqlx::query("UPDATE todos SET completed = 1 WHERE title = 'Buy groceries'")
            .execute(pool)
            .await?;

        // Seed Work todos (list_id = 2)
        for title in &[
            "Finish quarterly report",
            "Review pull requests",
            "Update documentation",
            "Team standup notes",
            "Deploy to staging",
        ] {
            sqlx::query("INSERT INTO todos (title, list_id) VALUES (?, 2)")
                .bind(title)
                .execute(pool)
                .await?;
        }
        // Mark some as completed
        sqlx::query("UPDATE todos SET completed = 1 WHERE title = 'Review pull requests'")
            .execute(pool)
            .await?;
        sqlx::query("UPDATE todos SET completed = 1 WHERE title = 'Team standup notes'")
            .execute(pool)
            .await?;

        println!("Seeded database with sample data.");
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// GET / - Show all lists
async fn index(State(pool): State<SqlitePool>) -> Result<Html<String>, AppError> {
    let rows: Vec<(i64, String, i64)> = sqlx::query_as(
        "SELECT l.id, l.name, COUNT(t.id) as todo_count
         FROM lists l
         LEFT JOIN todos t ON t.list_id = l.id
         GROUP BY l.id
         ORDER BY l.created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    let lists: Vec<ListWithCount> = rows
        .into_iter()
        .map(|(id, name, todo_count)| ListWithCount {
            id,
            name,
            todo_count,
        })
        .collect();

    let template = IndexTemplate { lists };
    Ok(Html(template.render()?))
}

/// POST /lists - Create a new list
async fn create_list(
    State(pool): State<SqlitePool>,
    Form(form): Form<CreateListForm>,
) -> Result<Redirect, AppError> {
    let name = form.name.trim().to_string();
    if !name.is_empty() {
        sqlx::query("INSERT INTO lists (name) VALUES (?)")
            .bind(&name)
            .execute(&pool)
            .await?;
    }
    Ok(Redirect::to("/"))
}

/// POST /lists/:id/delete - Delete a list
async fn delete_list(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    // Delete todos first (in case FK cascade isn't enforced)
    sqlx::query("DELETE FROM todos WHERE list_id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM lists WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(Redirect::to("/"))
}

/// GET /lists/:id - Show a list with its todos
async fn show_list(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let list: TodoList = sqlx::query_as("SELECT id, name, created_at FROM lists WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    let todos: Vec<TodoItem> = sqlx::query_as(
        "SELECT id, title, completed, list_id, created_at
         FROM todos
         WHERE list_id = ?
         ORDER BY completed ASC, created_at DESC",
    )
    .bind(id)
    .fetch_all(&pool)
    .await?;

    let completed_count = todos.iter().filter(|t| t.completed).count();

    let template = ListTemplate {
        list,
        todos,
        completed_count,
    };
    Ok(Html(template.render()?))
}

/// POST /lists/:id/todos - Create a todo in a list
async fn create_todo(
    State(pool): State<SqlitePool>,
    Path(list_id): Path<i64>,
    Form(form): Form<CreateTodoForm>,
) -> Result<Redirect, AppError> {
    let title = form.title.trim().to_string();
    if !title.is_empty() {
        sqlx::query("INSERT INTO todos (title, list_id) VALUES (?, ?)")
            .bind(&title)
            .bind(list_id)
            .execute(&pool)
            .await?;
    }
    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

/// POST /todos/:id/toggle - Toggle todo completed status
async fn toggle_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    // Get current state and list_id
    let (completed, list_id): (bool, i64) =
        sqlx::query_as("SELECT completed, list_id FROM todos WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await?;

    sqlx::query("UPDATE todos SET completed = ? WHERE id = ?")
        .bind(!completed)
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

/// POST /todos/:id/delete - Delete a todo
async fn delete_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let (list_id,): (i64,) = sqlx::query_as("SELECT list_id FROM todos WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

/// POST /todos/:id/edit - Edit a todo title
async fn edit_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
    Form(form): Form<EditTodoForm>,
) -> Result<Redirect, AppError> {
    let (list_id,): (i64,) = sqlx::query_as("SELECT list_id FROM todos WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    let title = form.title.trim().to_string();
    if !title.is_empty() {
        sqlx::query("UPDATE todos SET title = ? WHERE id = ?")
            .bind(&title)
            .bind(id)
            .execute(&pool)
            .await?;
    }

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create database pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:todo.db?mode=rwc")
        .await?;

    // Initialize tables and seed data
    init_db(&pool).await?;

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
        .with_state(pool);

    println!("Todo App running at http://localhost:5003");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5003").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
