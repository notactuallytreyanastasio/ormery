# Skinny Ecto

A simplified, in-memory implementation of [Ecto](https://hexdocs.pm/ecto/Ecto.html) (Elixir's database library) written in [Temper](https://github.com/temperlang/temper).

## What is Skinny Ecto?

Skinny Ecto demonstrates Temper's capabilities by implementing a composable query builder with schemas, records, and an in-memory data store. It showcases:

- **Schema Definition** - Type-safe field definitions with primary keys and nullability
- **In-Memory Store** - Simple key-value storage with auto-incrementing IDs
- **Query Builder** - Fluent, chainable query API with filtering, sorting, and pagination
- **Type Safety** - Full type checking and inference via Temper's type system

## Features

### Schema & Field Definition

```temper
let userFields = [
  field("name", "String", false, false),
  field("age", "Int", false, false),
  field("email", "String", false, true),
];
let userSchema = schema("users", userFields);
```

### Data Insertion

```temper
let store = new InMemoryStore();
store.insert("users", new Map<String, String>([
  new Pair("name", "Alice"),
  new Pair("age", "25"),
  new Pair("email", "alice@example.com"),
]));
```

### Query Building

```temper
// Filter, project, and limit
let adultNames = new Query(userSchema, store)
  .where("age", ">=", "18")
  .select(["name", "email"])
  .orderBy("age", "desc")
  .limit(10)
  .all();
```

### Supported Operations

- **Filtering**: `where(field, operator, value)` with `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Projection**: `select(fields)` to choose specific columns
- **Sorting**: `orderBy(field, direction)` with `asc`/`desc`
- **Pagination**: `limit(n)` and `offset(n)`
- **Chaining**: All operations return `this` for fluent composition

## Project Structure

```
skinny-ecto/
├── src/
│   ├── skinny-ecto.temper.md      # Main implementation (552 lines)
│   ├── config.temper.md           # Configuration types
│   ├── demo-controller.temper.md  # Interactive demo controller
│   └── syntax-highlighter.temper.md # Syntax highlighting
├── tutorial/
│   ├── index.html                 # Static tutorial site
│   ├── interactive.html           # Interactive playground
│   ├── demo.html                  # Live Temper demo
│   └── lib/                       # Compiled JavaScript modules
├── docs/
│   ├── graph-data.json            # Decision graph export
│   └── git-history.json           # Git history for graph
├── .deciduous/                    # Decision tracking
├── .claude/                       # Claude Code integration
└── run-python.py                  # Test runner
```

## Getting Started

### Prerequisites

- [Temper compiler](https://github.com/temperlang/temper) installed
- Node.js (for running compiled JavaScript)
- Python 3 (for test runner)

### Build & Run

```bash
# Compile Temper to JavaScript
temper build --backend js

# Run the demo
node temper.out/skinny-ecto.js

# Run tests
./run-python.py
```

### View Tutorials

```bash
cd tutorial
python3 -m http.server 8000
# Open http://localhost:8000
```

## Tutorial Sites

The project includes three tutorial experiences:

1. **Static Tutorial** (`index.html`) - Step-by-step guide with code examples
2. **Interactive Playground** (`interactive.html`) - Live code editor with instant feedback
3. **Real Temper Demo** (`demo.html`) - Uses actual compiled Temper JavaScript modules

## Decision Graph

This project uses [Deciduous](https://github.com/notactuallytreyanastasio/deciduous) to track development decisions, architecture choices, and implementation history.

```bash
# View decision graph locally
deciduous serve

# Export for GitHub Pages
deciduous sync
```

The graph captures the full development journey, including design decisions, implementation approaches, bug fixes, and refactoring choices.

## Implementation Details

### Type Safety

Skinny Ecto leverages Temper's type system for compile-time safety:

```temper
public getField(name: String): Field throws Bubble {
  for (let field of fields) {
    if (field.name == name) {
      return field;
    }
  }
  bubble()  // Type-safe error propagation
}
```

### Query Execution Pipeline

Queries follow a four-stage pipeline:

1. **Filter** - Apply `where` clauses
2. **Sort** - Apply `orderBy` clauses
3. **Slice** - Apply `offset` and `limit`
4. **Project** - Apply `select` fields

### Comparison Strategy

Type-aware comparison for Int and String fields:

```temper
let cmp = when (fieldType) {
  "Int" -> do {
    let aInt = aVal.toInt32() orelse 0;
    let bInt = bVal.toInt32() orelse 0;
    aInt <=> bInt
  };
  "String" -> aVal <=> bVal;
  else -> 0;
};
```

## Contributing

This project demonstrates Temper's capabilities and serves as a learning resource. Contributions welcome!

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests with `./run-python.py`
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Inspired by [Ecto](https://hexdocs.pm/ecto/Ecto.html) from the Elixir ecosystem
- Built with [Temper](https://github.com/temperlang/temper)
- Decision tracking via [Deciduous](https://github.com/notactuallytreyanastasio/deciduous)
