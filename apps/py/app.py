import os
import sys
import sqlite3
from datetime import datetime, timezone

# ---------------------------------------------------------------------------
# Vendor path setup – make Temper-compiled ORMery packages importable
# ---------------------------------------------------------------------------
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(BASE_DIR, "vendor", "temper-core"))
sys.path.insert(0, os.path.join(BASE_DIR, "vendor", "std"))
sys.path.insert(0, os.path.join(BASE_DIR, "vendor", "ormery"))

from flask import (
    Flask,
    abort,
    redirect,
    render_template,
    request,
    send_file,
    url_for,
)

from ormery.ormery import schema, field, Query, InMemoryStore, to_insert_sql
from temper_core import Pair, map_constructor

# ---------------------------------------------------------------------------
# App configuration
# ---------------------------------------------------------------------------
DB_PATH = os.path.join(BASE_DIR, "todos.db")

app = Flask(__name__, static_folder="static")
app.config["SECRET_KEY"] = "retro-todo-secret-key"

# ---------------------------------------------------------------------------
# ORMery Schemas
# ---------------------------------------------------------------------------
store = InMemoryStore()

list_schema = schema(
    "lists",
    [
        field("name", "String", False, False),
        field("created_at", "String", False, True),
    ],
)

todo_schema = schema(
    "todos",
    [
        field("title", "String", False, False),
        field("completed", "Int", False, False),
        field("list_id", "Int", False, False),
        field("created_at", "String", False, True),
    ],
)

# ---------------------------------------------------------------------------
# Database helpers
# ---------------------------------------------------------------------------


def get_db():
    """Open a connection to the SQLite database."""
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA foreign_keys=ON")
    return conn


def _now_iso():
    """Return the current UTC time as an ISO-8601 string."""
    return datetime.now(timezone.utc).isoformat()


def _make_map(pairs_dict):
    """Build a Temper MappingProxyType (Map) from a plain Python dict.

    The ORMery ``to_insert_sql`` function expects a ``MappingProxyType``
    built via ``map_constructor`` from ``temper_core``.  All values must be
    strings.
    """
    return map_constructor(tuple(Pair(k, v) for k, v in pairs_dict.items()))


# ---------------------------------------------------------------------------
# Serve shared retro.css from parent directory
# ---------------------------------------------------------------------------


@app.route("/retro.css")
def retro_css():
    css_path = os.path.join(BASE_DIR, "..", "retro.css")
    return send_file(css_path, mimetype="text/css")


# ---------------------------------------------------------------------------
# Simple namespace object so templates can use ``item.list.id``, etc.
# ---------------------------------------------------------------------------


class _Obj:
    """Tiny attribute-bag so templates keep working unchanged."""

    def __init__(self, **kw):
        self.__dict__.update(kw)


# ---------------------------------------------------------------------------
# Routes -- Lists
# ---------------------------------------------------------------------------


@app.route("/")
def index():
    """Show all lists."""
    db = get_db()
    try:
        # SELECT via ORMery
        sql = (
            Query(list_schema, store)
            .order_by("created_at", "asc")
            .to_sql()
            .to_string()
        )
        lists = db.execute(sql).fetchall()

        list_data = []
        for row in lists:
            total = db.execute(
                "SELECT COUNT(*) FROM todos WHERE list_id = ?", (row["id"],)
            ).fetchone()[0]
            done = db.execute(
                "SELECT COUNT(*) FROM todos WHERE list_id = ? AND completed = 1",
                (row["id"],),
            ).fetchone()[0]
            lst_obj = _Obj(id=row["id"], name=row["name"], created_at=row["created_at"])
            list_data.append({"list": lst_obj, "total": total, "done": done})

        return render_template("index.html", list_data=list_data)
    finally:
        db.close()


@app.route("/lists", methods=["POST"])
def create_list():
    """Create a new list."""
    name = request.form.get("name", "").strip()
    if name:
        db = get_db()
        try:
            values = _make_map({"name": name, "created_at": _now_iso()})
            sql = to_insert_sql(list_schema, values).to_string()
            db.execute(sql)
            db.commit()
        finally:
            db.close()
    return redirect(url_for("index"))


@app.route("/lists/<int:list_id>")
def show_list(list_id):
    """Show a single list with its todos."""
    db = get_db()
    try:
        row = db.execute("SELECT * FROM lists WHERE id = ?", (list_id,)).fetchone()
        if row is None:
            abort(404)
        lst = _Obj(id=row["id"], name=row["name"], created_at=row["created_at"])

        # SELECT todos via ORMery
        sql = (
            Query(todo_schema, store)
            .where("list_id", "==", str(list_id))
            .order_by("created_at", "asc")
            .to_sql()
            .to_string()
        )
        todo_rows = db.execute(sql).fetchall()

        todos = [
            _Obj(
                id=r["id"],
                title=r["title"],
                completed=bool(r["completed"]),
                list_id=r["list_id"],
                created_at=r["created_at"],
            )
            for r in todo_rows
        ]
        total = len(todos)
        done = sum(1 for t in todos if t.completed)
        return render_template("list.html", list=lst, todos=todos, total=total, done=done)
    finally:
        db.close()


@app.route("/lists/<int:list_id>/edit", methods=["POST"])
def edit_list(list_id):
    """Rename a list."""
    db = get_db()
    try:
        row = db.execute("SELECT * FROM lists WHERE id = ?", (list_id,)).fetchone()
        if row is None:
            abort(404)
        name = request.form.get("name", "").strip()
        if name:
            db.execute("UPDATE lists SET name = ? WHERE id = ?", (name, list_id))
            db.commit()
    finally:
        db.close()
    return redirect(url_for("index"))


@app.route("/lists/<int:list_id>/delete", methods=["POST"])
def delete_list(list_id):
    """Delete a list and all its todos."""
    db = get_db()
    try:
        row = db.execute("SELECT * FROM lists WHERE id = ?", (list_id,)).fetchone()
        if row is None:
            abort(404)
        db.execute("DELETE FROM todos WHERE list_id = ?", (list_id,))
        db.execute("DELETE FROM lists WHERE id = ?", (list_id,))
        db.commit()
    finally:
        db.close()
    return redirect(url_for("index"))


# ---------------------------------------------------------------------------
# Routes -- Todos
# ---------------------------------------------------------------------------


@app.route("/lists/<int:list_id>/todos", methods=["POST"])
def create_todo(list_id):
    """Add a todo to a list."""
    db = get_db()
    try:
        row = db.execute("SELECT * FROM lists WHERE id = ?", (list_id,)).fetchone()
        if row is None:
            abort(404)
        title = request.form.get("title", "").strip()
        if title:
            values = _make_map(
                {
                    "title": title,
                    "completed": "0",
                    "list_id": str(list_id),
                    "created_at": _now_iso(),
                }
            )
            sql = to_insert_sql(todo_schema, values).to_string()
            db.execute(sql)
            db.commit()
    finally:
        db.close()
    return redirect(url_for("show_list", list_id=list_id))


@app.route("/todos/<int:todo_id>/toggle", methods=["POST"])
def toggle_todo(todo_id):
    """Toggle the completed state of a todo."""
    db = get_db()
    try:
        todo = db.execute("SELECT * FROM todos WHERE id = ?", (todo_id,)).fetchone()
        if todo is None:
            abort(404)
        new_val = 0 if todo["completed"] else 1
        db.execute("UPDATE todos SET completed = ? WHERE id = ?", (new_val, todo_id))
        db.commit()
        list_id = todo["list_id"]
    finally:
        db.close()
    return redirect(url_for("show_list", list_id=list_id))


@app.route("/todos/<int:todo_id>/edit", methods=["POST"])
def edit_todo(todo_id):
    """Edit a todo's title."""
    db = get_db()
    try:
        todo = db.execute("SELECT * FROM todos WHERE id = ?", (todo_id,)).fetchone()
        if todo is None:
            abort(404)
        title = request.form.get("title", "").strip()
        if title:
            db.execute("UPDATE todos SET title = ? WHERE id = ?", (title, todo_id))
            db.commit()
        list_id = todo["list_id"]
    finally:
        db.close()
    return redirect(url_for("show_list", list_id=list_id))


@app.route("/todos/<int:todo_id>/delete", methods=["POST"])
def delete_todo(todo_id):
    """Delete a todo."""
    db = get_db()
    try:
        todo = db.execute("SELECT * FROM todos WHERE id = ?", (todo_id,)).fetchone()
        if todo is None:
            abort(404)
        list_id = todo["list_id"]
        db.execute("DELETE FROM todos WHERE id = ?", (todo_id,))
        db.commit()
    finally:
        db.close()
    return redirect(url_for("show_list", list_id=list_id))


# ---------------------------------------------------------------------------
# Database initialisation & seed data
# ---------------------------------------------------------------------------

DDL_LISTS = """\
CREATE TABLE IF NOT EXISTS lists (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    name       TEXT    NOT NULL,
    created_at TEXT
)"""

DDL_TODOS = """\
CREATE TABLE IF NOT EXISTS todos (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    title      TEXT    NOT NULL,
    completed  INTEGER NOT NULL DEFAULT 0,
    list_id    INTEGER NOT NULL REFERENCES lists(id),
    created_at TEXT
)"""


def init_db():
    """Create tables if they don't exist."""
    db = get_db()
    try:
        db.execute(DDL_LISTS)
        db.execute(DDL_TODOS)
        db.commit()
    finally:
        db.close()


def seed_database():
    """Populate empty database with sample data."""
    db = get_db()
    try:
        row = db.execute("SELECT id FROM lists LIMIT 1").fetchone()
        if row is not None:
            return  # Already seeded

        now = _now_iso()

        # Insert lists via ORMery
        personal_vals = _make_map({"name": "Personal", "created_at": now})
        db.execute(to_insert_sql(list_schema, personal_vals).to_string())

        work_vals = _make_map({"name": "Work", "created_at": now})
        db.execute(to_insert_sql(list_schema, work_vals).to_string())

        db.commit()

        personal_id = db.execute(
            "SELECT id FROM lists WHERE name = 'Personal'"
        ).fetchone()["id"]
        work_id = db.execute(
            "SELECT id FROM lists WHERE name = 'Work'"
        ).fetchone()["id"]

        # Insert todos via ORMery
        sample_todos = [
            ("Buy groceries", 0, personal_id),
            ("Call the dentist", 1, personal_id),
            ("Read a book", 0, personal_id),
            ("Go for a walk", 0, personal_id),
            ("Finish quarterly report", 0, work_id),
            ("Reply to client emails", 1, work_id),
            ("Update project roadmap", 0, work_id),
        ]
        for title, completed, lid in sample_todos:
            vals = _make_map(
                {
                    "title": title,
                    "completed": str(completed),
                    "list_id": str(lid),
                    "created_at": now,
                }
            )
            db.execute(to_insert_sql(todo_schema, vals).to_string())

        db.commit()
    finally:
        db.close()


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

if __name__ == "__main__":
    init_db()
    seed_database()
    app.run(debug=True, port=5001)
