# ORMery Lua Demo — LuaSocket + lsqlite3

A retro-styled todo list app built with a hand-rolled HTTP server (LuaSocket) and lsqlite3, using the **Temper-compiled ORMery query builder** for schema definition, SELECT queries, and INSERT operations.

Port: **5005**

## How ORMery Is Vendored

The compiled ORMery library lives in `vendor/` with three subdirectories:

```
vendor/
  ormery/          ← the query builder
  std/             ← Temper standard library
  temper-core/     ← Temper runtime
```

These are added to `package.path` in `db.lua`:

```lua
local script_dir = debug.getinfo(1, "S").source:match("^@(.*/)") or "./"
package.path = script_dir .. "vendor/ormery/?.lua;"
    .. script_dir .. "vendor/ormery/?/init.lua;"
    .. script_dir .. "vendor/temper-core/?.lua;"
    .. script_dir .. "vendor/temper-core/?/init.lua;"
    .. script_dir .. "vendor/std/?.lua;"
    .. script_dir .. "vendor/std/?/init.lua;"
    .. package.path

local temper = require("temper-core")
local ormery = require("ormery")
```

No frameworks, no ORMs — just LuaSocket for HTTP and lsqlite3 for SQLite.

## How ORMery Is Used

### Schema Definition

From `db.lua` — schemas defined at module load time. Lua uses `temper.listof()` to construct the field list.

```lua
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
```

### Helper Functions

Two helper functions bridge ORMery's API to Lua's table-based style:

```lua
-- Build a Temper Map from a plain Lua table
local function make_values(tbl)
    local pairs_list = {}
    for k, v in pairs(tbl) do
        pairs_list[#pairs_list + 1] = temper.pair_constructor(k, tostring(v))
    end
    return temper.map_constructor(pairs_list)
end

-- Build a SELECT query via ORMery, return SQL string
local function build_select(schema, opts)
    opts = opts or {}
    local q = ormery.Query(schema, nil)  -- store=nil; only call :toSql()

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

-- Build an INSERT via ORMery, return SQL string
local function build_insert(schema, values_table)
    local vals = make_values(values_table)
    return ormery.toInsertSql(schema, vals):toString()
end
```

### SELECT Queries (ORMery)

The `build_select` helper wraps ORMery's `Query` builder:

```lua
-- Get a list by ID
function db.lists_get_by_id(id)
    local sql = build_select(lists_schema, {
        where = {{"id", "=", id}},
        limit = 1,
    })
    for row in conn:nrows(sql) do return row end
    return nil
end

-- Get todos for a list, sorted
function db.todos_get_by_list(list_id)
    local sql = build_select(todos_schema, {
        where = {{"list_id", "=", list_id}},
        order = {{"completed", "asc"}, {"created_at", "desc"}},
    })
    local items = {}
    for row in conn:nrows(sql) do items[#items + 1] = row end
    return items
end
```

### INSERT Operations (ORMery)

The `build_insert` helper wraps `ormery.toInsertSql()`:

```lua
-- Create a new list
function db.lists_create(name)
    local sql = build_insert(lists_schema, { name = name })
    conn:exec(sql)
    return conn:last_insert_rowid()
end

-- Create a new todo
function db.todos_create(title, list_id)
    local sql = build_insert(todos_schema, {
        title     = title,
        completed = 0,
        list_id   = list_id,
    })
    conn:exec(sql)
    return conn:last_insert_rowid()
end

-- Seed data also uses ORMery INSERT
local sql = build_insert(lists_schema, { name = "Work Tasks" })
conn:exec(sql)
local work_id = conn:last_insert_rowid()
```

### Raw SQL (not supported by ORMery)

UPDATE, DELETE, and JOINs use prepared statements:

```lua
-- Toggle completed (ORMery doesn't generate UPDATE)
function db.todos_toggle(id)
    local stmt = conn:prepare(
        "UPDATE todos SET completed = 1 - completed WHERE id = ?")
    stmt:bind_values(id)
    stmt:step()
    stmt:finalize()
end

-- Delete a todo (ORMery doesn't generate DELETE)
function db.todos_delete(id)
    local stmt = conn:prepare("DELETE FROM todos WHERE id = ?")
    stmt:bind_values(id)
    stmt:step()
    stmt:finalize()
end

-- Index page aggregate (ORMery doesn't support JOINs)
function db.lists_get_all()
    local sql = [[
        SELECT l.*, COUNT(t.id) AS todo_count,
               SUM(CASE WHEN t.completed = 1 THEN 1 ELSE 0 END) AS done_count
        FROM lists l LEFT JOIN todos t ON t.list_id = l.id
        GROUP BY l.id ORDER BY l.created_at DESC
    ]]
    local items = {}
    for row in conn:nrows(sql) do items[#items + 1] = row end
    return items
end
```

## Running

```bash
lua app.lua
# → Server running on http://localhost:5005
```

Requires `luasocket` and `lsqlite3complete` (install via LuaRocks).
