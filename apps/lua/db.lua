-- db.lua - SQLite database layer using ORMery for query building
--
-- Uses the vendored ORMery library (compiled from Temper) for SELECT and
-- INSERT query generation.  UPDATE, DELETE, and DDL stay as raw SQL because
-- ORMery does not yet have builders for those statement types.

local sqlite3 = require("lsqlite3complete")

-- ── vendor path bootstrap ───────────────────────────────────────────
local script_dir = debug.getinfo(1, "S").source:match("^@(.*/)") or "./"
package.path = script_dir .. "vendor/ormery/?.lua;"
    .. script_dir .. "vendor/ormery/?/init.lua;"
    .. script_dir .. "vendor/temper-core/?.lua;"
    .. script_dir .. "vendor/temper-core/?/init.lua;"
    .. script_dir .. "vendor/std/?.lua;"
    .. script_dir .. "vendor/std/?/init.lua;"
    .. package.path

local temper  = require("temper-core")
local ormery  = require("ormery")

-- ── ORMery schemas ──────────────────────────────────────────────────
-- schema() auto-prepends an `id: Int (PK)` field.

local lists_schema = ormery.schema("lists", temper.listof(
    ormery.field("name",       "String", false, false),
    ormery.field("created_at", "String", false, true)
))

local todos_schema = ormery.schema("todos", temper.listof(
    ormery.field("title",      "String", false, false),
    ormery.field("completed",  "Int",    false, false),
    ormery.field("list_id",    "Int",    false, false),
    ormery.field("created_at", "String", false, true)
))

-- ── helpers ─────────────────────────────────────────────────────────

-- Build a temper Map from a plain Lua table of key/value pairs.
-- All values are stringified because ORMery treats field values as strings.
local function make_values(tbl)
    local pairs_list = {}
    for k, v in pairs(tbl) do
        pairs_list[#pairs_list + 1] = temper.pair_constructor(k, tostring(v))
    end
    return temper.map_constructor(pairs_list)
end

-- Build a SELECT query via ORMery and return the SQL string.
-- `opts` is an optional table: { where = {{f,op,v},...}, order = {{f,dir},...}, limit = n }
local function build_select(schema, opts)
    opts = opts or {}
    local q = ormery.Query(schema, nil)  -- store=nil; we only call :toSql()

    if opts.fields then
        q:select(temper.listof(table.unpack(opts.fields)))
    end

    if opts.where then
        for _, w in ipairs(opts.where) do
            q:where(w[1], w[2], tostring(w[3]))
        end
    end

    if opts.order then
        for _, o in ipairs(opts.order) do
            q:orderBy(o[1], o[2])
        end
    end

    if opts.limit then
        q:limit(opts.limit)
    end

    return q:toSql():toString()
end

-- Build an INSERT via ORMery and return the SQL string.
local function build_insert(schema, values_table)
    local vals = make_values(values_table)
    local fragment = ormery.toInsertSql(schema, vals)
    return fragment:toString()
end

-- ── module state ────────────────────────────────────────────────────
local db   = {}
local conn = nil

-- ── connection management ───────────────────────────────────────────

function db.open(path)
    path = path or (script_dir .. "todo.db")
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

-- ── DDL (raw SQL) ───────────────────────────────────────────────────

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

-- ── seed data ───────────────────────────────────────────────────────

function db.seed()
    -- Check existing count via ORMery SELECT
    local count_sql = build_select(lists_schema)
    local count = 0
    for _ in conn:nrows(count_sql) do count = count + 1 end
    if count > 0 then return false end

    conn:exec("BEGIN;")

    -- Insert lists using ORMery
    local sql = build_insert(lists_schema, { name = "Work Tasks" })
    conn:exec(sql)
    local work_id = conn:last_insert_rowid()

    sql = build_insert(lists_schema, { name = "Shopping List" })
    conn:exec(sql)
    local shop_id = conn:last_insert_rowid()

    -- Insert todos using ORMery
    local work_todos = {
        { "Review pull requests", 1 },
        { "Write unit tests",    0 },
        { "Update documentation", 0 },
        { "Fix login bug",       1 },
        { "Deploy to staging",   0 },
    }
    for _, t in ipairs(work_todos) do
        sql = build_insert(todos_schema, {
            title     = t[1],
            completed = t[2],
            list_id   = work_id,
        })
        conn:exec(sql)
    end

    local shop_todos = {
        { "Milk",         1 },
        { "Bread",        0 },
        { "Eggs",         0 },
        { "Coffee beans", 1 },
        { "Bananas",      0 },
    }
    for _, t in ipairs(shop_todos) do
        sql = build_insert(todos_schema, {
            title     = t[1],
            completed = t[2],
            list_id   = shop_id,
        })
        conn:exec(sql)
    end

    conn:exec("COMMIT;")
    return true
end

-- ════════════════════════════════════════════════════════════════════
-- Lists CRUD
-- ════════════════════════════════════════════════════════════════════

function db.lists_create(name)
    local sql = build_insert(lists_schema, { name = name })
    conn:exec(sql)
    return conn:last_insert_rowid()
end

function db.lists_get_all()
    -- This query uses a LEFT JOIN + GROUP BY which is beyond ORMery's
    -- current scope, so we keep it as raw SQL.
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
            id         = stmt:get_value(0),
            name       = stmt:get_value(1),
            created_at = stmt:get_value(2),
            total      = stmt:get_value(3),
            done       = stmt:get_value(4) or 0,
        })
    end
    stmt:finalize()
    return results
end

function db.lists_get_by_id(id)
    -- ORMery SELECT with WHERE id = ?
    local sql = build_select(lists_schema, {
        where = { { "id", "==", id } },
        limit = 1,
    })
    local result = nil
    for row in conn:nrows(sql) do
        result = {
            id         = row.id,
            name       = row.name,
            created_at = row.created_at,
        }
        break
    end
    return result
end

function db.lists_update(id, name)
    -- UPDATE not supported by ORMery; raw SQL
    local stmt = conn:prepare("UPDATE lists SET name = ? WHERE id = ?")
    stmt:bind_values(name, id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

function db.lists_delete(id)
    -- DELETE not supported by ORMery; raw SQL
    local stmt = conn:prepare("DELETE FROM lists WHERE id = ?")
    stmt:bind_values(id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

-- ════════════════════════════════════════════════════════════════════
-- Todos CRUD
-- ════════════════════════════════════════════════════════════════════

function db.todos_create(title, list_id)
    local sql = build_insert(todos_schema, {
        title   = title,
        list_id = list_id,
    })
    conn:exec(sql)
    return conn:last_insert_rowid()
end

function db.todos_get_by_list(list_id)
    -- ORMery SELECT with WHERE + ORDER BY
    local sql = build_select(todos_schema, {
        where = { { "list_id", "==", list_id } },
        order = { { "completed", "asc" }, { "created_at", "desc" } },
    })
    local results = {}
    for row in conn:nrows(sql) do
        table.insert(results, {
            id         = row.id,
            title      = row.title,
            completed  = row.completed,
            list_id    = row.list_id,
            created_at = row.created_at,
        })
    end
    return results
end

function db.todos_get_by_id(id)
    local sql = build_select(todos_schema, {
        where = { { "id", "==", id } },
        limit = 1,
    })
    local result = nil
    for row in conn:nrows(sql) do
        result = {
            id         = row.id,
            title      = row.title,
            completed  = row.completed,
            list_id    = row.list_id,
            created_at = row.created_at,
        }
        break
    end
    return result
end

function db.todos_update(id, title)
    -- UPDATE not supported by ORMery; raw SQL
    local stmt = conn:prepare("UPDATE todos SET title = ? WHERE id = ?")
    stmt:bind_values(title, id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

function db.todos_toggle(id)
    -- UPDATE not supported by ORMery; raw SQL
    local stmt = conn:prepare("UPDATE todos SET completed = 1 - completed WHERE id = ?")
    stmt:bind_values(id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

function db.todos_delete(id)
    -- DELETE not supported by ORMery; raw SQL
    local stmt = conn:prepare("DELETE FROM todos WHERE id = ?")
    stmt:bind_values(id)
    stmt:step()
    stmt:finalize()
    return conn:changes() > 0
end

function db.todos_get_list_id(id)
    -- ORMery SELECT for just the list_id field
    local sql = build_select(todos_schema, {
        fields = { "list_id" },
        where  = { { "id", "==", id } },
        limit  = 1,
    })
    local list_id = nil
    for row in conn:nrows(sql) do
        list_id = row.list_id
        break
    end
    return list_id
end

return db
