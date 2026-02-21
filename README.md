# ORMery

A simplified, in-memory implementation of [Ecto](https://hexdocs.pm/ecto/Ecto.html) (Elixir's database library) written in [Temper](https://github.com/temperlang/temper). Demonstrates composable query building, type-safe schemas, and literate programming.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Features](#features)
- [Project Structure](#project-structure)
- [Building](#building)
- [Running the Demo](#running-the-demo)
- [Tutorial Sites](#tutorial-sites)
- [API Reference](#api-reference)
- [Query Execution Pipeline](#query-execution-pipeline)
- [Source Code Guide](#source-code-guide)
- [Decision Graph](#decision-graph)
- [Deployment](#deployment)
- [Contributing](#contributing)
- [License](#license)

## Overview

ORMery showcases what Temper can do by implementing a real query builder pattern from scratch. It compiles to both JavaScript and Python, runs in the browser via pre-compiled modules, and ships with three interactive tutorial sites.

**What you get:**

- Schema definition with typed fields, primary keys, and nullability
- An in-memory key-value store with auto-incrementing IDs
- A fluent, chainable query API (filter, sort, project, paginate)
- Type-aware comparisons (Int vs String)
- Literate programming source (`.temper.md` files mixing prose and code)
- Three tutorial experiences: static lessons, interactive playground, compiled demo

## Quick Start

```bash
# Check dependencies
make deps

# Build everything and prepare browser bundle
make startup

# Start the tutorial server
make serve

# In another terminal, open the interactive playground
make open
```

Or manually:

```bash
temper build --backend js            # Compile
node temper.out/js/ormery/ormery.js            # Run CLI demo
cd tutorial && python3 -m http.server 8000     # Serve tutorials
# Open http://localhost:8000/interactive.html
```

## Features

### Schema & Field Definition

Define tables with typed fields. An `id` primary key is added automatically.

```temper
let userFields = [
  field("name", "String", false, false),
  field("age", "Int", false, false),
  field("email", "String", false, true),   // nullable
];
let userSchema = schema("users", userFields);
```

### Data Insertion

Store records in an in-memory store with auto-incrementing IDs.

```temper
let store = new InMemoryStore();
store.insert("users", new Map<String, String>([
  new Pair("name", "Alice"),
  new Pair("age", "25"),
  new Pair("email", "alice@example.com"),
]));
```

### Query Building

Chain operations to build queries. All methods return `this` for fluent composition.

```temper
let adultNames = new Query(userSchema, store)
  .where("age", ">=", "18")
  .select(["name", "email"])
  .orderBy("age", "desc")
  .limit(10)
  .all();
```

### Supported Operations

| Operation | Method | Description |
|-----------|--------|-------------|
| Filter | `.where(field, op, value)` | Filter records. Operators: `==`, `!=`, `>`, `<`, `>=`, `<=` |
| Project | `.select(fields)` | Choose specific columns to return |
| Sort | `.orderBy(field, dir)` | Sort by field. Direction: `"asc"` or `"desc"` |
| Limit | `.limit(n)` | Return at most `n` records |
| Offset | `.offset(n)` | Skip the first `n` records |
| Execute | `.all()` | Run the query and return results |

Multiple `.where()` calls combine with AND logic. Multiple `.orderBy()` calls apply in order (primary sort, secondary sort, etc).

## Project Structure

```
ormery/
├── Makefile                          # Build, run, serve, deploy commands
├── src/
│   ├── config.temper.md              # Library configuration (name export)
│   ├── ormery.temper.md              # Core implementation (537 lines)
│   │   ├── Field & Schema            #   Type-safe table definitions
│   │   ├── Record & InMemoryStore    #   Storage with auto-increment IDs
│   │   ├── Query Builder             #   Chainable query API
│   │   ├── Comparison Helpers        #   Type-aware Int/String comparisons
│   │   └── Demo (main)              #   11 example queries
│   ├── demo-controller.temper.md     # Business logic for interactive demo
│   └── syntax-highlighter.temper.md  # Temper syntax highlighter in Temper
├── tutorial/
│   ├── index.html                    # Static tutorial (8 lessons)
│   ├── interactive.html              # Interactive playground with editor
│   ├── demo.html                     # Live demo using compiled Temper JS
│   ├── serve.sh                      # Quick-start server script
│   ├── build-bundle.sh               # Compile & create browser bundle
│   ├── bundle-temper.sh              # Copy compiled JS to tutorial/lib
│   ├── README.md                     # Tutorial-specific documentation
│   └── lib/                          # Pre-compiled JavaScript modules
│       ├── ormery/                   #   Compiled ORMery (v0.6.0)
│       │   ├── ormery.js            #   Main module (~27 KB)
│       │   ├── ormery.js.map        #   Source map
│       │   ├── index.js              #   Re-export entry point
│       │   └── package.json          #   Module metadata
│       └── temper-core/              #   Temper runtime library (v0.6.0)
│           ├── index.js              #   Entry point
│           ├── core.js               #   Core runtime
│           ├── string.js             #   String operations
│           ├── async.js              #   Async support
│           ├── bitvector.js          #   Bit vectors
│           ├── deque.js              #   Double-ended queues
│           ├── listed.js             #   List collections
│           ├── mapped.js             #   Map collections
│           ├── pair.js               #   Key-value pairs
│           ├── regex.js              #   Regular expressions
│           ├── check-type.js         #   Type checking
│           ├── interface.js          #   Interface support
│           ├── float.js              #   Floating point
│           ├── date.js               #   Date/time
│           ├── net.js                #   Network
│           ├── package.json          #   @temperlang/core metadata
│           └── tsconfig.json         #   TypeScript config
├── docs/                             # GitHub Pages deployment
│   ├── index.html                    # Decision graph viewer (compiled)
│   ├── graph-data.json               # Decision graph export
│   ├── git-history.json              # Git history for graph nodes
│   └── .nojekyll                     # Disable Jekyll on GitHub Pages
├── .deciduous/                       # Decision tracking
│   ├── config.toml                   # Branch detection settings
│   ├── deciduous.db                  # SQLite decision database
│   └── documents/                    # Document snapshots
├── .claude/                          # Claude Code integration
│   ├── settings.json                 # Hook configuration
│   ├── agents.toml                   # Subagent definitions
│   ├── commands/                     # Custom slash commands (9 files)
│   ├── hooks/                        # Pre/post tool hooks (2 scripts)
│   └── skills/                       # Archaeology, narratives, pulse
├── .github/workflows/
│   ├── deploy-pages.yml              # Deploy docs/ to GitHub Pages
│   └── cleanup-decision-graphs.yml   # Clean up PR visualization artifacts
├── run-python.py                     # Python test runner
└── .gitignore
```

## Building

### Prerequisites

| Tool | Required | Purpose |
|------|----------|---------|
| [Temper](https://github.com/temperlang/temper) | Yes | Compile `.temper.md` source files |
| [Node.js](https://nodejs.org/) | Yes | Run compiled JavaScript |
| [Python 3](https://python.org/) | Yes | Run tests, serve tutorial |
| [Deciduous](https://github.com/notactuallytreyanastasio/deciduous) | Optional | Decision graph tracking |

Check what you have installed:

```bash
make deps
```

### Makefile Targets

```bash
make help          # Show all available targets

# Build
make build         # Compile Temper to JavaScript
make build-js      # Same as build
make build-py      # Compile Temper to Python
make bundle        # Build + copy JS modules to tutorial/lib

# Run
make run           # Run CLI demo (Node.js)
make run-py        # Run CLI demo (Python)
make test          # Run tests (Python backend)

# Tutorial & Demo
make serve         # Start tutorial server on :8000
make open          # Open interactive playground in browser
make open-tutorial # Open static tutorial in browser
make open-demo     # Open Temper-compiled demo in browser

# Decision Graph
make graph         # View nodes and edges
make graph-serve   # Start graph viewer
make graph-sync    # Export for GitHub Pages
make graph-check   # Audit for orphaned nodes

# Cleanup
make clean         # Remove all build artifacts
make clean-build   # Remove temper.out/
make clean-bundle  # Remove bundled JS from tutorial/lib

# All-in-One
make all           # Build + bundle
make startup       # Check deps + build + bundle
```

Custom port: `make serve PORT=3000`

### Manual Build Steps

```bash
# JavaScript backend
temper build --backend js
node temper.out/js/ormery/ormery.js

# Python backend
temper build --backend py
./run-python.py

# Bundle for browser
mkdir -p tutorial/lib
cp -r temper.out/js/temper-core tutorial/lib/
cp -r temper.out/js/ormery tutorial/lib/
```

## Running the Demo

### CLI Demo

The main source file includes 11 demonstration queries:

```bash
make run
```

Output:

```
=== ORMery Demo ===

Schema: users
  - id: Int (PK)
  - name: String
  - age: Int
  - email: String (nullable)

Inserted 3 users:
  id: 1, name: Alice, age: 25, email: alice@example.com
  id: 2, name: Bob, age: 30, email: bob@example.com
  id: 3, name: Charlie, age: 17, email: charlie@example.com

=== Query 1: All users ===
=== Query 2: Users age >= 18 ===
=== Query 3: Just names and ages ===
=== Query 4: Adult names only ===
=== Query 5: Age >= 18 AND age < 30 ===
=== Query 6: All users ordered by age (asc) ===
=== Query 7: All users ordered by age (desc) ===
=== Query 8: First 2 users (limit) ===
=== Query 9: Skip first user (offset) ===
=== Query 10: Page 2, size 1 (offset + limit) ===
=== Query 11: Oldest adult (where + orderBy + limit) ===
```

### Tutorial Server

```bash
make serve    # Starts on http://localhost:8000
make open     # Opens interactive playground
```

## Tutorial Sites

Three ways to explore ORMery, from reading to hands-on:

### 1. Static Tutorial (`index.html`)

Eight progressive lessons with pre-rendered code examples. Best for reading and understanding the concepts.

**Lessons covered:**
1. What is ORMery?
2. Defining schemas and fields
3. Creating an in-memory store
4. Inserting records
5. Basic queries
6. Filtering with WHERE
7. Sorting and pagination
8. Putting it all together

### 2. Interactive Playground (`interactive.html`)

**The recommended starting point.** A full in-browser IDE with three tabs:

- **Code Editor** - Write and run ORMery code with live output
- **Query Builder** - Visual form-based query construction
- **Examples** - 8 ready-to-run examples demonstrating all features

Features:
- Split-pane editor (Code | Output)
- Click-to-load examples
- Add/remove WHERE clauses dynamically
- SELECT field checkboxes
- ORDER BY, LIMIT, OFFSET controls
- Copy generated code from builder to editor
- No external dependencies, runs entirely in-browser

### 3. Temper-Compiled Demo (`demo.html`)

Uses the actual Temper-compiled JavaScript modules from `tutorial/lib/`. Demonstrates real integration with Temper's compiled output rather than a JavaScript reimplementation.

### Design

All three tutorials share a purple gradient theme (`#667eea` to `#764ba2`) with responsive layouts that work on all screen sizes.

## API Reference

### `field(name, fieldType, primaryKey, nullable): Field`

Create a field definition.

```temper
field("email", "String", false, true)   // nullable String field
field("id", "Int", true, false)         // primary key Int field
```

**Parameters:**
- `name: String` - Column name
- `fieldType: String` - `"String"` or `"Int"`
- `primaryKey: Boolean` - Whether this is the primary key
- `nullable: Boolean` - Whether null values are allowed

### `schema(tableName, fields): Schema`

Create a schema. Automatically prepends an `id: Int (PK)` field.

```temper
let s = schema("users", [
  field("name", "String", false, false),
]);
// Schema has fields: [id (PK), name]
```

### `Schema`

| Method | Returns | Description |
|--------|---------|-------------|
| `.getField(name)` | `Field` | Look up a field by name (throws on missing) |
| `.hasField(name)` | `Boolean` | Check if field exists |
| `.primaryKeyField` | `Field` | Get the primary key field (throws if none) |
| `.fieldNames` | `List<String>` | List all field names |
| `.describe()` | `String` | Human-readable schema description |

### `InMemoryStore`

| Method | Returns | Description |
|--------|---------|-------------|
| `.insert(table, data)` | `Record` | Insert a record (ID assigned automatically) |
| `.all(table)` | `List<Record>` | Get all records in a table |
| `.get(table, id)` | `Record` | Get a record by ID (throws on missing) |
| `.count(table)` | `Int` | Count records in a table |

### `Record`

| Method | Returns | Description |
|--------|---------|-------------|
| `.get(field)` | `String` | Get field value (throws on missing) |
| `.getOr(field, fallback)` | `String` | Get field value with default |
| `.has(field)` | `Boolean` | Check if field exists |
| `.id` | `Int` | Get the record's ID |
| `.describe()` | `String` | Human-readable `"key: value, ..."` |

### `Query`

Construct with `new Query(schema, store)`, chain methods, execute with `.all()`.

| Method | Returns | Description |
|--------|---------|-------------|
| `.where(field, op, value)` | `Query` | Add a filter clause |
| `.select(fields)` | `Query` | Set field projection |
| `.orderBy(field, direction)` | `Query` | Add sort clause (`"asc"` / `"desc"`) |
| `.limit(n)` | `Query` | Set maximum results |
| `.offset(n)` | `Query` | Skip first `n` results |
| `.all()` | `List<Record>` | Execute and return results |

**Where operators:** `==`, `!=`, `>`, `<`, `>=`, `<=`

## Query Execution Pipeline

Queries execute in four stages, always in this order:

```
Records → Filter → Sort → Slice → Project → Results
```

1. **Filter** - Apply all `.where()` clauses (AND logic). Each clause checks the record's field value against the clause value using type-aware comparison.

2. **Sort** - Apply `.orderBy()` clauses in declaration order. Comparison is type-aware: Int fields compare numerically, String fields compare lexicographically.

3. **Slice** - Apply `.offset()` then `.limit()` to the sorted results for pagination.

4. **Project** - If `.select()` was called, build new records containing only the requested fields. Otherwise return full records.

### Type-Aware Comparisons

The query engine inspects the schema to determine field types:

```temper
// Int fields: numeric comparison
// "25" > "8" (because 25 > 8 numerically)

// String fields: lexicographic comparison
// "Alice" < "Bob" (because 'A' < 'B')
```

This prevents the common pitfall where `"8" > "25"` in naive string comparison.

## Source Code Guide

All source is written in Temper's literate programming format (`.temper.md`), where prose and code are interleaved in Markdown files. Code blocks indented with 4 spaces (in `ormery.temper.md`) or fenced with triple backticks are compiled; everything else is documentation.

### `src/ormery.temper.md` (537 lines)

The core library. Sections:

| Section | Lines | What it defines |
|---------|-------|-----------------|
| Field and Schema | 1-79 | `Field` class, `Schema` class, `field()` and `schema()` constructors |
| Record and Store | 80-165 | `Record` class, `InMemoryStore` class with insert/get/all/count |
| Query Builder | 167-374 | `WhereClause`, `OrderClause`, `Query` class, comparison helpers |
| Demo | 376-537 | `main()` with 11 example queries |

### `src/demo-controller.temper.md` (139 lines)

A `DemoController` class for the interactive tutorial demo. Wraps query execution with formatted output and query counting. Contains 5 pre-built demo scenarios.

### `src/syntax-highlighter.temper.md` (276 lines)

A syntax highlighter for Temper code, written in Temper. Tokenizes source into keywords, types, strings, numbers, comments, operators, and identifiers. Outputs HTML with CSS class spans for styling.

### `src/config.temper.md` (11 lines)

Library configuration. Exports `name = "ormery"` for the Temper build system.

## Decision Graph

This project tracks every design decision, implementation action, and outcome using [Deciduous](https://github.com/notactuallytreyanastasio/deciduous).

```bash
make graph          # View all nodes and edges
make graph-serve    # Interactive graph viewer (auto-refreshes)
make graph-sync     # Export for GitHub Pages
make graph-check    # Audit for disconnected nodes
```

The decision graph captures:
- **Goals** - What we set out to build and why
- **Decisions** - Choices between approaches (with confidence levels)
- **Actions** - What was implemented and which commits are linked
- **Outcomes** - What worked, what failed, and lessons learned
- **Observations** - Things noticed during development

The graph is deployed to GitHub Pages automatically when `docs/` changes are pushed to `main`.

## Deployment

### Tutorial Sites

The tutorial is static HTML with no build step. Deploy to any static host:

```bash
# GitHub Pages (from tutorial/ directory)
# Set Pages source to /tutorial in repo settings

# Or any static host
# Upload the tutorial/ directory
```

### Decision Graph (GitHub Pages)

Automatic via GitHub Actions:

```bash
make graph-sync                    # Export graph data
git add docs/graph-data.json docs/git-history.json docs/index.html
git commit -m "docs: update decision graph"
git push                           # Triggers deploy-pages.yml
```

The workflow at `.github/workflows/deploy-pages.yml` deploys `docs/` to the `gh-pages` branch using `peaceiris/actions-gh-pages@v4`.

Manual setup:
1. Go to Settings > Pages
2. Set source to "Deploy from a branch"
3. Select `gh-pages` branch, `/ (root)` folder

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes to `.temper.md` source files
4. Build and test: `make build && make run`
5. Run Python tests: `make test`
6. Submit a pull request

### Code Style

- Use literate programming format (`.temper.md`)
- Follow Temper naming conventions (PascalCase for types, camelCase for functions)
- Document public APIs in the prose sections
- Include examples in markdown sections
- Test all query operations

### Git Rules

Always stage files explicitly:

```bash
# Good
git add src/ormery.temper.md tutorial/interactive.html

# Bad - never do this
git add .
git add -A
```

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- Inspired by [Ecto](https://hexdocs.pm/ecto/Ecto.html) from the Elixir ecosystem
- Built with [Temper](https://github.com/temperlang/temper)
- Decision tracking via [Deciduous](https://github.com/notactuallytreyanastasio/deciduous)
