# ORMery

A query builder and ORM in [Temper](https://github.com/temperlang/temper) that compiles to **six languages** with injection-proof SQL generation.

Write the ORM once in Temper. Get native libraries for JavaScript, Python, C#, Rust, Java, and Lua — same API, same safety guarantees, same generated SQL. Every demo app on this page uses the *same* compiled ORMery library for schema definition, SELECT queries, and INSERT operations.

```
.where("name", "=", "Robert'); DROP TABLE users;--")
 => SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'

.where("1=1; DROP TABLE users;--", "=", "Alice")
 => SELECT * FROM users                            (field rejected — not in schema)
```

**[View the full project documentation →](https://notactuallytreyanastasio.github.io/ormery/)**

## Quick Start

```bash
make startup                       # deps + build + bundle
make run                           # CLI demo
make serve                         # tutorial on :8000
```

Or manually:

```bash
temper build --backend js
node -e "require('./temper.out/js/ormery/ormery.js').main()"
```

## How It Works

### Define a schema

```temper
let userSchema = schema("users", [
  field("name", "String", false, false),
  field("age", "Int", false, false),
  field("email", "String", false, true),
]);
```

An `id: Int (PK)` field is prepended automatically.

### Query + generate SQL

```temper
let q = new Query(userSchema, store)
  .where("age", ">=", "18")
  .select(["name", "email"])
  .orderBy("age", "desc")
  .limit(10);

q.toSql().toString()
// => SELECT name, email FROM users WHERE age >= 18 ORDER BY age DESC LIMIT 10
```

### Insert with autoescaping

```temper
toInsertSql(userSchema, new Map<String, String>([
  new Pair("name", "O'Malley"),
  new Pair("age", "42"),
])).toString()
// => INSERT INTO users (name, age) VALUES ('O''Malley', 42)
```

## ORMery in Six Languages

The same Temper source compiles to six native libraries. Here's what the schema + query API looks like in each:

### JavaScript
```javascript
import { schema, field, Query, InMemoryStore, toInsertSql } from 'ormery';
const store = new InMemoryStore();

const todoSchema = schema("todos", [
  field("title", "String", false, false),
  field("completed", "Int", false, false),
  field("list_id", "Int", false, false),
]);

// SELECT
const sql = new Query(todoSchema, store)
  .where("list_id", "=", String(id))
  .orderBy("completed", "asc")
  .toSql().toString();
const todos = db.prepare(sql).all();

// INSERT
const insertSql = toInsertSql(todoSchema, new Map([["title", title]])).toString();
db.prepare(insertSql).run();
```

### Python
```python
from ormery.ormery import schema, field, Query, InMemoryStore, to_insert_sql
from temper_core import Pair, map_constructor

store = InMemoryStore()
todo_schema = schema("todos", [
    field("title", "String", False, False),
    field("completed", "Int", False, False),
    field("list_id", "Int", False, False),
])

# SELECT
sql = (Query(todo_schema, store)
    .where("list_id", "=", str(list_id))
    .order_by("completed", "asc")
    .to_sql().to_string())
todos = conn.execute(sql).fetchall()

# INSERT
vals = map_constructor(tuple(Pair(k, v) for k, v in {"title": title}.items()))
conn.execute(to_insert_sql(todo_schema, vals).to_string())
```

### C#
```csharp
using Ormery;
using TemperLang.Core;

var todoSchema = OrmeryGlobal.Schema("todos",
    Listed.CreateReadOnlyList<Field>(
        OrmeryGlobal.Field("title", "String", false, false),
        OrmeryGlobal.Field("completed", "Int", false, false),
        OrmeryGlobal.Field("list_id", "Int", false, false)
    ));

// SELECT
string sql = new Query(todoSchema, store)
    .Where("list_id", "=", listId.ToString())
    .OrderBy("completed", "asc")
    .ToSql().ToString();

// INSERT
var values = Mapped.ConstructMap(
    Listed.CreateReadOnlyList<KeyValuePair<string, string>>(
        new KeyValuePair<string, string>("title", title)));
string insertSql = OrmeryGlobal.ToInsertSql(todoSchema, values).ToString();
```

### Rust
```rust
use ormery::{schema, field, Query, InMemoryStore, to_insert_sql};
use std::sync::Arc;

let todo_schema = schema("todos".to_string(), Arc::new(vec![
    field("title".to_string(), "String".to_string(), false, false),
    field("completed".to_string(), "Int".to_string(), false, false),
    field("list_id".to_string(), "Int".to_string(), false, false),
]));

// SELECT — note r#where (where is a Rust keyword)
let sql = Query::new(todo_schema.clone(), store.clone())
    .r#where("list_id", "=", list_id.to_string())
    .order_by("completed", "asc")
    .to_sql().to_string().to_string();

// INSERT
let values = temper_core::Map::new(&[
    (Arc::new("title".to_string()), Arc::new(title.to_string())),
]);
let insert_sql = to_insert_sql(todo_schema.clone(), values)
    .to_string().to_string();
```

### Java
```java
import ormery.OrmeryGlobal;
import ormery.Query;
import ormery.Schema;

Schema todoSchema = OrmeryGlobal.schema("todos", List.of(
    OrmeryGlobal.field("title", "String", false, false),
    OrmeryGlobal.field("completed", "Int", false, false),
    OrmeryGlobal.field("list_id", "Int", false, false)
));

// SELECT
String sql = new Query(todoSchema, dummyStore)
    .where("list_id", "=", String.valueOf(listId))
    .orderBy("completed", "asc")
    .toSql().toString();

// INSERT
Map<String, String> values = new LinkedHashMap<>();
values.put("title", title);
jdbc.execute(OrmeryGlobal.toInsertSql(todoSchema, values).toString());
```

### Lua
```lua
local ormery = require("ormery")
local temper = require("temper-core")

local todo_schema = ormery.schema("todos", temper.listof(
    ormery.field("title",     "String", false, false),
    ormery.field("completed", "Int",    false, false),
    ormery.field("list_id",   "Int",    false, false)
))

-- SELECT
local q = ormery.Query(todo_schema, nil)
q:where("list_id", "=", tostring(list_id))
q:orderBy("completed", "asc")
local sql = q:toSql():toString()

-- INSERT
local vals = temper.map_constructor({
    temper.pair_constructor("title", title)
})
conn:exec(ormery.toInsertSql(todo_schema, vals):toString())
```

## Security Model

SQL generation uses the `sql"..."` tagged string macro from secure-composition. The Temper compiler desugars interpolations into typed `SqlBuilder.append*` calls at compile time, so the escaping is structural, not bolted on.

**Values** (user input) flow through `sql"${value}"`, which dispatches to `appendString` (quotes + escapes `'`) or `appendInt32` (no quotes) based on the interpolated type. You cannot forget to escape.

**Identifiers** (table names, column names, operators) are passed through `appendSafe`, which marks them as trusted SQL source. To prevent confused deputy attacks:

1. `toSqlQuery` validates every field name in `selectFields`, `whereClauses`, and `orderClauses` against `schema.hasField()` — only declared fields pass through.
2. `validOperator` allowlists comparison operators (`=`, `!=`, `<>`, `>`, `<`, `>=`, `<=`). Unknown operators fall back to `=`.
3. Table names come from `schema.tableName`, which is developer-defined.

### Test coverage

35 tests (5 upstream SQL + 30 ORMery):

- Bobby Tables injection blocked (SELECT and INSERT)
- Embedded quotes escaped (`O'Brien` => `O''Brien`)
- Empty strings, Unicode, operator normalization
- Adversarial field names in WHERE, SELECT, ORDER BY silently dropped
- Adversarial table names in schema creation blocked
- Non-numeric Int values produce always-false conditions
- Limit zero emits `LIMIT 0`, negative limits clamped

## API

### Schema definition

| Function | Description |
|----------|-------------|
| `field(name, type, primaryKey, nullable)` | Create a field. Types: `"String"`, `"Int"` |
| `schema(tableName, fields)` | Create a schema (auto-prepends `id` PK) |

### Query builder

Construct with `new Query(schema, store)`, chain methods, execute with `.all()` or generate SQL with `.toSql()`.

| Method | Returns | Description |
|--------|---------|-------------|
| `.where(field, op, value)` | `Query` | Add filter. Ops: `=` `!=` `>` `<` `>=` `<=` |
| `.select(fields)` | `Query` | Project to specific columns |
| `.orderBy(field, dir)` | `Query` | Sort. Direction: `"asc"` or `"desc"` |
| `.limit(n)` | `Query` | Cap result count |
| `.offset(n)` | `Query` | Skip first `n` results |
| `.all()` | `List<Record>` | Execute against in-memory store |
| `.toSql()` | `SqlFragment` | Generate SQL (does not execute) |

### SQL generation

| Function | Description |
|----------|-------------|
| `toInsertSql(schema, values)` | Build an INSERT fragment with autoescaped values |

### What ORMery handles vs. raw SQL

| Operation | ORMery? | Notes |
|-----------|---------|-------|
| SELECT | Yes | `.where()`, `.orderBy()`, `.limit()`, `.select()` |
| INSERT | Yes | `toInsertSql(schema, values)` with autoescaping |
| UPDATE | No | Use raw parameterized SQL or `SqlBuilder` |
| DELETE | No | Use raw parameterized SQL or `SqlBuilder` |
| CREATE TABLE | No | Raw DDL |
| JOINs / Aggregates | No | Raw SQL |

## End-to-End Pipeline

A single push to this repo triggers a fully automated pipeline:

```
ormery (push to main)
  │
  ▼
publish-libs.yml
  │
  ├── temper build (JDK 21 + Gradle)
  │     produces temper.out/{js,py,csharp,rust,java,lua}/
  │
  └── 6 parallel matrix jobs
        │
        ├──▶ ormery-js       ──notify-app.yml──▶ ormery-js-app/vendor/
        ├──▶ ormery-py       ──notify-app.yml──▶ ormery-py-app/vendor/
        ├──▶ ormery-csharp   ──notify-app.yml──▶ ormery-csharp-app/TodoApp/vendor/
        ├──▶ ormery-rust     ──notify-app.yml──▶ ormery-rust-app/vendor/
        ├──▶ ormery-java     ──notify-app.yml──▶ ormery-java-app/vendor/
        └──▶ ormery-lua      ──notify-app.yml──▶ ormery-lua-app/vendor/

Result: 12 repos updated from a single push
```

**Tier 1** (`publish-libs.yml`): Builds Temper, pushes compiled output (full tree: `ormery/` + `std/` + `temper-core/`) to six lib repos via SSH deploy keys.

**Tier 2** (`notify-app.yml`): Each lib repo push triggers a workflow that clones the corresponding app repo, replaces `vendor/{ormery,std,temper-core}/` with the latest compiled output, and pushes.

## Satellite Repositories

### Compiled Libraries

Each contains the full compiled Temper tree. Automatically updated by the pipeline.

| Repo | Language |
|------|----------|
| [ormery-js](https://github.com/notactuallytreyanastasio/ormery-js) | JavaScript |
| [ormery-py](https://github.com/notactuallytreyanastasio/ormery-py) | Python |
| [ormery-csharp](https://github.com/notactuallytreyanastasio/ormery-csharp) | C# |
| [ormery-rust](https://github.com/notactuallytreyanastasio/ormery-rust) | Rust |
| [ormery-java](https://github.com/notactuallytreyanastasio/ormery-java) | Java |
| [ormery-lua](https://github.com/notactuallytreyanastasio/ormery-lua) | Lua |

### Demo Apps

Retro-styled todo list managers (Mac System 6 + Windows 95 UI), each using ORMery + a thin SQLite driver.

| Repo | Stack | Port |
|------|-------|------|
| [ormery-js-app](https://github.com/notactuallytreyanastasio/ormery-js-app) | Express + ORMery + better-sqlite3 + EJS | 5006 |
| [ormery-py-app](https://github.com/notactuallytreyanastasio/ormery-py-app) | Flask + ORMery + sqlite3 + Jinja2 | 5001 |
| [ormery-csharp-app](https://github.com/notactuallytreyanastasio/ormery-csharp-app) | ASP.NET Core + ORMery + Microsoft.Data.Sqlite + Razor | 5002 |
| [ormery-rust-app](https://github.com/notactuallytreyanastasio/ormery-rust-app) | Axum + ORMery + rusqlite + Askama | 5003 |
| [ormery-java-app](https://github.com/notactuallytreyanastasio/ormery-java-app) | Spring Boot + ORMery + JDBC + Thymeleaf | 5004 |
| [ormery-lua-app](https://github.com/notactuallytreyanastasio/ormery-lua-app) | LuaSocket + ORMery + lsqlite3 | 5005 |

## Project Structure

```
src/
  config.temper.md              Library name ("ormery")
  ormery.temper.md              Schema, Store, Query, SQL gen, tests, demo
  demo-controller.temper.md     Tutorial demo scenarios
  syntax-highlighter.temper.md  Temper syntax highlighter
  sql/                          secure-composition SQL module (embedded)
    model.temper.md             SqlFragment, SqlPart types, escaping
    builder.temper.md           SqlBuilder, sql"..." tag, append overloads
    tests.temper.md             Upstream SQL escaping tests
apps/
  js/                           JavaScript demo app (Express + EJS)
  py/                           Python demo app (Flask + Jinja2)
  csharp/                       C# demo app (ASP.NET Core + Razor)
  rust/                         Rust demo app (Axum + Askama)
  java/                         Java demo app (Spring Boot + Thymeleaf)
  lua/                          Lua demo app (LuaSocket)
  retro.css                     Shared retro CSS theme
docs/
  index.html                    GitHub Pages site
tutorial/
  index.html                    Static tutorial (8 lessons)
  interactive.html              Interactive playground
  demo.html                     Live demo using compiled Temper JS
```

All Temper source is in literate programming format: `.temper.md` files where 4-space-indented blocks are code and everything else is prose.

## Building

**Prerequisites:** [Temper](https://github.com/temperlang/temper), [Node.js](https://nodejs.org/), [Python 3](https://python.org/)

```bash
make deps              # check what's installed
make build             # compile to JS
make run               # run CLI demo
make bundle            # copy compiled JS to tutorial/lib
make serve             # tutorial server on :8000
make clean             # remove build artifacts
```

`temper build` compiles to `temper.out/{js,py,csharp,rust,java,lua}/`. Tests run at build time — the build fails if any assertion fails.

## Acknowledgments

- Inspired by [Ecto](https://hexdocs.pm/ecto/Ecto.html) from the Elixir ecosystem
- SQL escaping via contextual autoescaping
- Built with [Temper](https://github.com/temperlang/temper)
- Decision tracking via [Deciduous](https://github.com/notactuallytreyanastasio/deciduous)
