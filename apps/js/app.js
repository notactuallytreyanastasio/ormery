import express from 'express';
import path from 'path';
import { fileURLToPath } from 'url';
import { createRequire } from 'module';
import { schema, field, Query, InMemoryStore, toInsertSql } from 'ormery';

const require = createRequire(import.meta.url);
const Database = require('better-sqlite3');

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const app = express();
const PORT = 5006;

// --- ORMery Schemas ---
const store = new InMemoryStore(); // dummy store for SQL generation

const listSchema = schema("lists", [
  field("name", "String", false, false),
  field("created_at", "String", false, true),
]);

const todoSchema = schema("todos", [
  field("title", "String", false, false),
  field("completed", "Int", false, false),
  field("list_id", "Int", false, false),
  field("created_at", "String", false, true),
]);

// --- Middleware ---
app.set('view engine', 'ejs');
app.set('views', path.join(__dirname, 'views'));
app.use(express.urlencoded({ extended: true }));
app.use(express.static(path.join(__dirname, 'public')));

// --- Database Setup ---
const db = new Database(path.join(__dirname, 'todo.db'));
db.pragma('journal_mode = WAL');
db.pragma('foreign_keys = ON');

db.exec(`
  CREATE TABLE IF NOT EXISTS lists (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TEXT DEFAULT (datetime('now'))
  )
`);

db.exec(`
  CREATE TABLE IF NOT EXISTS todos (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    completed INTEGER DEFAULT 0,
    list_id INTEGER REFERENCES lists(id),
    created_at TEXT DEFAULT (datetime('now'))
  )
`);

// --- Seed Data ---
const listCount = db.prepare('SELECT COUNT(*) AS cnt FROM lists').get().cnt;
if (listCount === 0) {
  const insertList = db.prepare('INSERT INTO lists (name) VALUES (?)');
  const insertTodo = db.prepare('INSERT INTO todos (title, completed, list_id) VALUES (?, ?, ?)');

  const seedDb = db.transaction(() => {
    const personal = insertList.run('Personal');
    const work = insertList.run('Work');

    insertTodo.run('Buy groceries', 0, personal.lastInsertRowid);
    insertTodo.run('Clean the apartment', 0, personal.lastInsertRowid);
    insertTodo.run('Read a book', 1, personal.lastInsertRowid);
    insertTodo.run('Go for a run', 0, personal.lastInsertRowid);

    insertTodo.run('Finish quarterly report', 0, work.lastInsertRowid);
    insertTodo.run('Email the team', 1, work.lastInsertRowid);
    insertTodo.run('Prepare slide deck', 0, work.lastInsertRowid);
  });

  seedDb();
}

// --- Prepared Statements (raw SQL for UPDATE/DELETE/aggregate) ---
const stmts = {
  // Aggregate/JOIN query - kept as raw SQL (ORMery doesn't support JOINs/subqueries)
  allLists: db.prepare(`
    SELECT l.*,
      (SELECT COUNT(*) FROM todos WHERE list_id = l.id) AS todo_count,
      (SELECT COUNT(*) FROM todos WHERE list_id = l.id AND completed = 1) AS done_count
    FROM lists l ORDER BY l.created_at DESC
  `),
  deleteList: db.prepare('DELETE FROM lists WHERE id = ?'),
  deleteTodosByList: db.prepare('DELETE FROM todos WHERE list_id = ?'),
  toggleTodo: db.prepare('UPDATE todos SET completed = CASE WHEN completed = 1 THEN 0 ELSE 1 END WHERE id = ?'),
  deleteTodo: db.prepare('DELETE FROM todos WHERE id = ?'),
  editTodo: db.prepare('UPDATE todos SET title = ? WHERE id = ?'),
  countTodosByList: db.prepare('SELECT COUNT(*) AS total, SUM(completed) AS done FROM todos WHERE list_id = ?'),
};

// --- Routes ---

// GET / - Show all lists
app.get('/', (req, res) => {
  const lists = stmts.allLists.all();
  res.render('index', { lists });
});

// POST /lists - Create a new list
app.post('/lists', (req, res) => {
  const name = (req.body.name || '').trim();
  if (name) {
    const vals = new Map([["name", name]]);
    const sql = toInsertSql(listSchema, vals).toString();
    db.prepare(sql).run();
  }
  res.redirect('/');
});

// POST /lists/:id/delete - Delete a list and its todos
app.post('/lists/:id/delete', (req, res) => {
  const id = parseInt(req.params.id, 10);
  const deleteListAndTodos = db.transaction((listId) => {
    stmts.deleteTodosByList.run(listId);
    stmts.deleteList.run(listId);
  });
  deleteListAndTodos(id);
  res.redirect('/');
});

// GET /lists/:id - Show a single list with its todos
app.get('/lists/:id', (req, res) => {
  const id = parseInt(req.params.id, 10);
  // Get a single list by ID using ORMery
  const listSql = new Query(listSchema, store).where("id", "=", String(id)).toSql().toString();
  const list = db.prepare(listSql).get();
  if (!list) {
    return res.redirect('/');
  }
  // Get todos for this list using ORMery
  const todosSql = new Query(todoSchema, store)
    .where("list_id", "=", String(id))
    .orderBy("completed", "asc")
    .orderBy("created_at", "desc")
    .toSql().toString();
  const todos = db.prepare(todosSql).all();
  const counts = stmts.countTodosByList.get(id);
  res.render('list', { list, todos, counts });
});

// POST /lists/:id/todos - Add a todo to a list
app.post('/lists/:id/todos', (req, res) => {
  const id = parseInt(req.params.id, 10);
  const title = (req.body.title || '').trim();
  if (title) {
    const vals = new Map([["title", title], ["list_id", String(id)]]);
    const sql = toInsertSql(todoSchema, vals).toString();
    db.prepare(sql).run();
  }
  res.redirect(`/lists/${id}`);
});

// POST /todos/:id/toggle - Toggle a todo's completed status
app.post('/todos/:id/toggle', (req, res) => {
  const id = parseInt(req.params.id, 10);
  // Get a single todo by ID using ORMery
  const todoSql = new Query(todoSchema, store).where("id", "=", String(id)).toSql().toString();
  const todo = db.prepare(todoSql).get();
  if (todo) {
    stmts.toggleTodo.run(id);
    res.redirect(`/lists/${todo.list_id}`);
  } else {
    res.redirect('/');
  }
});

// POST /todos/:id/delete - Delete a todo
app.post('/todos/:id/delete', (req, res) => {
  const id = parseInt(req.params.id, 10);
  // Get a single todo by ID using ORMery
  const todoSql = new Query(todoSchema, store).where("id", "=", String(id)).toSql().toString();
  const todo = db.prepare(todoSql).get();
  if (todo) {
    stmts.deleteTodo.run(id);
    res.redirect(`/lists/${todo.list_id}`);
  } else {
    res.redirect('/');
  }
});

// POST /todos/:id/edit - Edit a todo's title
app.post('/todos/:id/edit', (req, res) => {
  const id = parseInt(req.params.id, 10);
  const title = (req.body.title || '').trim();
  // Get a single todo by ID using ORMery
  const todoSql = new Query(todoSchema, store).where("id", "=", String(id)).toSql().toString();
  const todo = db.prepare(todoSql).get();
  if (todo && title) {
    stmts.editTodo.run(title, id);
    res.redirect(`/lists/${todo.list_id}`);
  } else if (todo) {
    res.redirect(`/lists/${todo.list_id}`);
  } else {
    res.redirect('/');
  }
});

// --- Start Server ---
app.listen(PORT, () => {
  console.log(`Todo app running at http://localhost:${PORT}`);
});
