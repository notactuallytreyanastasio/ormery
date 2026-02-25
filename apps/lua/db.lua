-- db.lua - SQLite database setup and ORM-like functions
local sqlite3 = require("lsqlite3complete")

local db = {}
local conn = nil

-- Get the directory of the current script
local function script_dir()
    local info = debug.getinfo(1, "S")
    local path = info.source:match("^@(.*/)")
    return path or "./"
end

function db.open(path)
    path = path or (script_dir() .. "todo.db")
    conn = sqlite3.open(path)
    conn:exec("PRAGMA journal_mode=WAL;")
    conn:exec("PRAGMA foreign_keys=ON;")
    return conn
end

function db.close()
    if conn then
        conn:close()
        conn = nil
    end
end

function db.get_conn()
    return conn
end

-- Create tables
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

-- Seed sample data if tables are empty
function db.seed()
    local stmt = conn:prepare("SELECT COUNT(*) FROM lists")
    stmt:step()
    local count = stmt:get_value(0)
    stmt:finalize()

    if count > 0 then
        return false -- already seeded
    end

    conn:exec("BEGIN;")

    -- Create two lists
    conn:exec([[INSERT INTO lists (name) VALUES ('Work Tasks');]])
    local work_id = conn:last_insert_rowid()

    conn:exec([[INSERT INTO lists (name) VALUES ('Shopping List');]])
    local shop_id = conn:last_insert_rowid()

    -- Work tasks
    local work_todos = {
        { "Review pull requests", 1 },
        { "Write unit tests", 0 },
        { "Update documentation", 0 },
        { "Fix login bug", 1 },
        { "Deploy to staging", 0 },
    }
    local ins = conn:prepare("INSERT INTO todos (title, completed, list_id) VALUES (?, ?, ?)")
    for _, t in ipairs(work_todos) do
        ins:bind_values(t[1], t[2], work_id)
        ins:step()
        ins:reset()
    end

    -- Shopping list
    local shop_todos = {
        { "Milk", 1 },
        { "Bread", 0 },
        { "Eggs", 0 },
        { "Coffee beans", 1 },
        { "Bananas", 0 },
    }
    for _, t in ipairs(shop_todos) do
        ins:bind_values(t[1], t[2], shop_id)
        ins:step()
        ins:reset()
    end
    ins:finalize()

    conn:exec("COMMIT;")
    return true
end

-- ============================================================
-- Lists CRUD
-- ============================================================

function db.lists_create(name)
    local stmt = conn:prepare("INSERT INTO lists (name) VALUES (?)")
    stmt:bind_values(name)
    stmt:step()
    stmt:finalize()
    return conn:last_insert_rowid()
end

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
            created_at = stmt:get_value(2),
            total = stmt:get_value(3),
            done = stmt:get_value(4) or 0,
        })
    end
    stmt:finalize()
    return results
end

function db.lists_get_by_id(id)
    local stmt = conn:prepare("SELECT id, name, created_at FROM lists WHERE id = ?")
    stmt:bind_values(id)
    local result = nil
    if stmt:step() == sqlite3.ROW then
        result = {
            id = stmt:get_value(0),
            name = stmt:get_value(1),
            created_at = stmt:get_value(2),
        }
    end
    stmt:finalize()
    return result
end

function db.lists_update(id, name)
    local stmt = conn:prepare("UPDATE lists SET name = ? WHERE id = ?")
    stmt:bind_values(name, id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

function db.lists_delete(id)
    local stmt = conn:prepare("DELETE FROM lists WHERE id = ?")
    stmt:bind_values(id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

-- ============================================================
-- Todos CRUD
-- ============================================================

function db.todos_create(title, list_id)
    local stmt = conn:prepare("INSERT INTO todos (title, list_id) VALUES (?, ?)")
    stmt:bind_values(title, list_id)
    stmt:step()
    stmt:finalize()
    return conn:last_insert_rowid()
end

function db.todos_get_by_list(list_id)
    local results = {}
    local stmt = conn:prepare([[
        SELECT id, title, completed, list_id, created_at
        FROM todos
        WHERE list_id = ?
        ORDER BY completed ASC, created_at DESC
    ]])
    stmt:bind_values(list_id)
    while stmt:step() == sqlite3.ROW do
        table.insert(results, {
            id = stmt:get_value(0),
            title = stmt:get_value(1),
            completed = stmt:get_value(2),
            list_id = stmt:get_value(3),
            created_at = stmt:get_value(4),
        })
    end
    stmt:finalize()
    return results
end

function db.todos_get_by_id(id)
    local stmt = conn:prepare("SELECT id, title, completed, list_id, created_at FROM todos WHERE id = ?")
    stmt:bind_values(id)
    local result = nil
    if stmt:step() == sqlite3.ROW then
        result = {
            id = stmt:get_value(0),
            title = stmt:get_value(1),
            completed = stmt:get_value(2),
            list_id = stmt:get_value(3),
            created_at = stmt:get_value(4),
        }
    end
    stmt:finalize()
    return result
end

function db.todos_update(id, title)
    local stmt = conn:prepare("UPDATE todos SET title = ? WHERE id = ?")
    stmt:bind_values(title, id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

function db.todos_toggle(id)
    local stmt = conn:prepare("UPDATE todos SET completed = 1 - completed WHERE id = ?")
    stmt:bind_values(id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

function db.todos_delete(id)
    local stmt = conn:prepare("DELETE FROM todos WHERE id = ?")
    stmt:bind_values(id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

-- Get the list_id for a todo (for redirects)
function db.todos_get_list_id(id)
    local stmt = conn:prepare("SELECT list_id FROM todos WHERE id = ?")
    stmt:bind_values(id)
    local list_id = nil
    if stmt:step() == sqlite3.ROW then
        list_id = stmt:get_value(0)
    end
    stmt:finalize()
    return list_id
end

return db
