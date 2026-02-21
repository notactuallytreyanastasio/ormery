# ORMery

A query builder and ORM in [Temper](https://github.com/temperlang/temper) with injection-proof SQL generation powered by [secure-composition](https://github.com/nickreserved/secure-composition).

ORMery pairs an in-memory query engine (filter, sort, project, paginate) with a pure SQL generation layer where **values are escaped automatically** by the `sql"..."` tagged string macro and **identifiers are validated against the schema whitelist** before reaching trusted SQL positions. The result: you can't produce injectable SQL through the public API.

```
.where("name", "=", "Robert'); DROP TABLE users;--")
 => SELECT * FROM users WHERE name = 'Robert''); DROP TABLE users;--'

.where("1=1; DROP TABLE users;--", "=", "Alice")
 => SELECT * FROM users                            (field rejected — not in schema)
```

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

### Insert records

```temper
let store = new InMemoryStore();
store.insert("users", new Map<String, String>([
  new Pair("name", "Alice"),
  new Pair("age", "25"),
  new Pair("email", "alice@example.com"),
]));
```

### Query in memory

```temper
let adults = new Query(userSchema, store)
  .where("age", ">=", "18")
  .select(["name", "email"])
  .orderBy("age", "desc")
  .limit(10)
  .all();
```

### Generate SQL

The same query state produces a `SqlFragment` via `toSql()`:

```temper
let q = new Query(userSchema, store)
  .select(["name", "age"])
  .where("age", ">=", "18")
  .orderBy("age", "desc")
  .limit(10);

q.toSql().toString()
// => SELECT name, age FROM users WHERE age >= 18 ORDER BY age DESC LIMIT 10
```

INSERT generation is also supported:

```temper
toInsertSql(userSchema, new Map<String, String>([
  new Pair("name", "O'Malley"),
  new Pair("age", "42"),
])).toString()
// => INSERT INTO users (name, age) VALUES ('O''Malley', 42)
```

## Security Model

SQL generation uses the `sql"..."` tagged string macro from secure-composition. The Temper compiler desugars interpolations into typed `SqlBuilder.append*` calls at compile time, so the escaping is structural, not bolted on.

**Values** (user input) flow through `sql"${value}"`, which dispatches to `appendString` (quotes + escapes `'`) or `appendInt32` (no quotes) based on the interpolated type. You cannot forget to escape.

**Identifiers** (table names, column names, operators) are passed through `appendSafe`, which marks them as trusted SQL source. To prevent confused deputy attacks where user-controlled strings reach identifier positions:

1. `toSqlQuery` validates every field name in `selectFields`, `whereClauses`, and `orderClauses` against `schema.hasField()` — only declared fields pass through.
2. `validOperator` allowlists comparison operators (`=`, `!=`, `<>`, `>`, `<`, `>=`, `<=`). Unknown operators fall back to `=`.
3. Table names come from `schema.tableName`, which is developer-defined.

The net effect: the only strings that reach `appendSafe` are schema field names, schema table names, and allowlisted operator literals.

### Test coverage

27 tests (5 upstream SQL + 22 ORMery):

- Bobby Tables injection blocked (SELECT and INSERT)
- Embedded quotes escaped (`O'Brien` => `O''Brien`)
- Empty strings, Unicode, operator normalization
- Adversarial field names in WHERE, SELECT, ORDER BY silently dropped

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
| `.where(field, op, value)` | `Query` | Add filter. Ops: `==` `!=` `>` `<` `>=` `<=` |
| `.select(fields)` | `Query` | Project to specific columns |
| `.orderBy(field, dir)` | `Query` | Sort. Direction: `"asc"` or `"desc"` |
| `.limit(n)` | `Query` | Cap result count |
| `.offset(n)` | `Query` | Skip first `n` results |
| `.all()` | `List<Record>` | Execute against in-memory store |
| `.toSql()` | `SqlFragment` | Generate SQL (does not execute) |

### SQL generation (pure functions)

| Function | Description |
|----------|-------------|
| `toSqlQuery(schema, select, where, order, limit, offset)` | Build a SELECT fragment |
| `toInsertSql(schema, values)` | Build an INSERT fragment |

### Storage

| Method | Description |
|--------|-------------|
| `store.insert(table, data)` | Insert record, returns `Record` with auto ID |
| `store.all(table)` | All records in table |
| `store.get(table, id)` | Lookup by ID (throws on miss) |
| `store.count(table)` | Record count |

## Query Pipeline

```
store.all() => Filter (.where) => Sort (.orderBy) => Slice (.offset/.limit) => Project (.select)
```

Comparisons are type-aware: Int fields compare numerically, String fields compare lexicographically. Multiple `.where()` calls combine with AND.

## Project Structure

```
src/
  config.temper.md              Library name ("ormery")
  ormery.temper.md              Schema, Store, Query, SQL gen, tests, demo (879 lines)
  demo-controller.temper.md     Tutorial demo scenarios
  syntax-highlighter.temper.md  Temper syntax highlighter with HTML escaping
  sql/                          secure-composition SQL module (embedded)
    model.temper.md             SqlFragment, SqlPart types, escaping
    builder.temper.md           SqlBuilder, sql"..." tag, append overloads
    tests.temper.md             Upstream SQL escaping tests
    imports.temper.md           Date import for SQL date type
tutorial/
  index.html                   Static tutorial (8 lessons)
  interactive.html             Interactive playground with editor
  demo.html                    Live demo using compiled Temper JS
  lib/                         Pre-compiled JS modules for browser
```

All source is in Temper's literate programming format: `.temper.md` files where 4-space-indented blocks are code and everything else is prose.

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

`temper build --backend js` compiles to `temper.out/js/ormery/ormery.js`. Tests run at build time — the build fails if any assertion fails.

## Tutorial

Three variants, all served from `tutorial/`:

1. **Static** (`index.html`) — 8 progressive lessons, read-only
2. **Interactive** (`interactive.html`) — code editor, visual query builder, click-to-run examples
3. **Compiled demo** (`demo.html`) — uses actual Temper-compiled JS modules

```bash
make serve && make open    # starts server and opens interactive playground
```

## Acknowledgments

- Inspired by [Ecto](https://hexdocs.pm/ecto/Ecto.html) from the Elixir ecosystem
- SQL escaping by [secure-composition](https://github.com/nickreserved/secure-composition)
- Built with [Temper](https://github.com/temperlang/temper)
- Decision tracking via [Deciduous](https://github.com/notactuallytreyanastasio/deciduous)
