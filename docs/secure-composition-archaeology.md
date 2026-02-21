# secure-composition: Archaeology & Integration Analysis

## Repository

- **Repo:** `temperlang/secure-composition` (private)
- **Description:** A Temper library for securely composing content in a variety of glue formats: HTML, SQL, file paths, shell strings, etc.
- **Age:** ~3 weeks (Jan 29 - Feb 20, 2026)
- **Commits:** 40 across 7 branches
- **Tests:** 27 passing (`temper test -b js`)
- **No releases tagged yet**

---

## 1. What It Is

secure-composition implements **contextual auto-escaping** — a technique where
a compiler-driven state machine tracks the parsing context of a structured
language (HTML, SQL, etc.) and automatically applies the correct escaping for
each interpolation point.

The core insight: instead of requiring developers to remember which escaping
function to call in which context, the library tracks context automatically and
the Temper compiler optimizes it at compile time.

```
sql"SELECT * FROM users WHERE name = ${userName}"
```

The `sql` tag tells the compiler to route this through `SqlBuilder`, which
produces a `SqlFragment` — a semi-structured representation where the literal
SQL is `SqlSource` (trusted) and the interpolated `userName` becomes
`SqlString` (escaped with single-quote doubling).

---

## 2. Product History (Chronological)

### Phase 1: Path Tag Foundation (Jan 29 - Feb 2)

The project started with **secure file path composition** — preventing CWE-22
path traversal attacks.

**Key design decisions:**
- Lexical-only path resolution (no filesystem access, no I/O delays)
- `..` blocked in interpolations by default (opt-in with `/<` prefix)
- Interpolations can never become absolute paths
- Bad paths collapse to `/dev/null/zz_Temper_zz` as an identifiable error marker
- Cross-platform: internal POSIX representation, renders to POSIX, Windows, or `file://` URLs

**Alongside this**, the core `contextual-autoescaping` framework was sketched:
- `Context` interface for tracking parser state
- `AutoescState<CONTEXT>` bundles context with nested language delegation
- `ContextPropagator` / `EscaperPicker` as pure functional operators
- Designed for compile-time inlining and optimization

### Phase 2: HTML Transition Tables & Codegen (Feb 3-5)

The most ambitious component. A **Python code generator** (`scripts/regen_temper/`)
reads transition tables from markdown comments in `.temper.md` files and produces
Temper state machine code.

**HTML parsing states (11 total):**

| State | Name | Where you are |
|-------|------|---------------|
| 0 | `htmlStatePcdata` | Text content outside tags |
| 1 | `htmlStateOName` | Inside opening tag name |
| 2 | `htmlStateCName` | Inside closing tag name |
| 3 | `htmlStateBeforeAttr` | Before attribute name |
| 4 | `htmlStateBeforeEq` | After attr name, before `=` |
| 5 | `htmlStateBeforeValue` | After `=`, before value |
| 6 | `htmlStateAttr` | Inside attribute value |
| 7 | `htmlStateAfterAttr` | After attribute value |
| 8 | `htmlStateSpecialBody` | Inside `<script>` or `<style>` |
| 9 | `htmlStateComment` | Inside `<!-- -->` |
| 10 | `htmlStateBogusComment` | Inside `<!` or `<?` bogus comments |

**Attribute context awareness:**
- Generic attributes: HTML entity escaping
- `style` attributes: CSS context delegation
- `on*` attributes: JS context (space escaper — blocks interpolation)
- `src`, `href`: URL context with protocol whitelist
- `srcset`: Multiple URL context

The HTML codec handles named entities (full WhatWG spec table), numeric
entities (`&#123;`, `&#xABC;`), and encoding of `&<>"'\0`.

### Phase 3: HTML Contextual Autoescaping Working (Feb 5-7)

**Key milestones:**
- `84b2cf9` — "safe-html passes first contextual auto-escaping testcase"
- `6f708e2` — "implemented chained escapers to get URL escaping inside HTML working"
- Nested language handling via `Delegate` + `Codec` pattern
- `NoTagEndCodec` prevents `</script>` injection in nested JS/CSS

**Escaper hierarchy:**
- `HtmlPcdataEscaper` — text content (`&lt;`, `&gt;`, `&amp;`)
- `HtmlAttributeEscaper` — attribute values with entity encoding
- `OutputHtmlSpaceEscaper` — outputs space for error/weird contexts
- `OutputZzTemperZzHtmlEscaper` — identifiable error marker
- `AllowSafeCssHtmlEscaper` — only allows `SafeCss`, blocks everything else
- URL escapers: protocol whitelist (http, https, mailto), percent-encoding

### Phase 4: SQL Module & Refinements (Feb 8-12)

SQL support added via two PRs:

**PR #1 — Initial SQL escaping (Feb 4):** Basic `sql` tagged string support.

**PR #2 — Semi-structured SQL representation (Feb 12):** The `SqlFragment` /
`SqlPart` sealed hierarchy providing typed escaping.

**Concurrent refinements:**
- `dc288f8` — Replaced `AutomatonStack` with flat subsidiary (simplification)
- `3a46ad1` — Escapers made pure functions (enables compile-time optimization)
- `76e3fad`, `1f049b8` — Tweaks for "macro magic" (compiler integration)

### Phase 5: Abstraction, Lists, CSS (Feb 13-20)

**PR #5 — Abstract part collection (Feb 16):** Extracted `Collector<PART>` for
reuse across different autoescaping contexts.

**PR #7 — SQL list append overloading (Feb 17):** Added `appendXxxList` methods
for all SQL types. Lists render as comma-separated values:
`sql"v IN (${[1, 2, 3]})"` → `"v IN (1, 2, 3)"`

**PR #8 — CSS handling (Feb 18):** The largest single change (+3,050 lines).
CSS state machine with 6 states, `url()` handling delegates to URL escaper,
`NoTagEndCodec` prevents `</style>` injection.

**Branch `tests-for-css-delegate` (Feb 20):** Active debugging of CSS context
propagation — the most recent work.

---

## 3. Architecture

### Module Map

```
secure-composition/
├── src/
│   ├── config.temper.md                    # Overview, planned modules table
│   ├── core/
│   │   ├── contextual-autoescaping.temper.md  # Generic framework
│   │   └── collector.temper.md                # Part collection helper
│   ├── html/
│   │   ├── safe-html.temper.md                # HTML autoescaper (23 touches!)
│   │   ├── html-codec.temper.md               # Entity encode/decode
│   │   ├── css-delegate.temper.md             # CSS nested language
│   │   ├── no-tag-end-codec.temper.md         # Prevents </tag> injection
│   │   └── transition-functions.temper.md     # Generated state machine code
│   ├── sql/
│   │   ├── imports.temper.md                  # Date from std/temporal
│   │   ├── model.temper.md                    # SqlFragment, SqlPart hierarchy
│   │   ├── builder.temper.md                  # SqlBuilder (the `sql` tag)
│   │   └── tests.temper.md                    # 27 tests
│   ├── path/
│   │   └── path.temper.md                     # Secure file path composition
│   ├── url/
│   │   └── url-codec.temper.md                # Percent-encoding utilities
│   └── examples/
│       ├── usage.temper.md                    # HTML link example
│       └── corner-cases.temper.md             # Comment edge cases
└── scripts/
    └── regen_temper/__main__.py               # Transition table codegen
```

### Planned But Not Yet Implemented

From `config.temper.md`:

| Module | Tags | Types | Status |
|--------|------|-------|--------|
| `./path` | `path` | `FilePath` | Done |
| `./html` | `html` | `SafeHtml`, `SafeUrl` | Done |
| `./sql` | `mysql`, `postgres` | `SafeSql` | Partial — `sql` tag works, dialect-specific tags and `SafeSql` wrapper not yet |
| `./shell` | `bash` | `SafeShell` | Not started |

### The Core Abstraction

```
Context (interface)
  └── tracks parser state in a streaming language

AutoescState<CONTEXT>
  └── context + optional Subsidiary (nested language delegate)

ContextPropagator<CONTEXT>
  └── processes trusted string chunks, transitions context

EscaperPicker<CONTEXT, ESC>
  └── returns the right escaper for current context

ContextualAutoescapingAccumulator<CONTEXT, ESC>
  └── coordinates propagation and escaping
  └── appendSafe(str) — trusted content
  └── append(value) — untrusted, gets escaped per context
```

All operators are **pure functions** designed for **compile-time inlining**.

---

## 4. SQL Module Deep Dive

### Type Hierarchy

```
SqlPart (sealed interface)
├── SqlSource(source: String)       — known-safe SQL text
├── SqlString(value: String)        — user data, single-quote escaped
├── SqlInt32(value: Int32)          — integer literal
├── SqlInt64(value: Int64)          — 64-bit integer literal
├── SqlFloat64(value: Float64)      — float literal
├── SqlBoolean(value: Boolean)      — TRUE / FALSE
└── SqlDate(value: Date)            — 'YYYY-MM-DD'

SqlFragment(parts: List<SqlPart>)
├── .toString(): String             — renders all parts
└── .toSource(): SqlSource          — freezes to trusted source
```

### SqlBuilder API

The `sql` tag is an alias for `SqlBuilder`:

```temper
export let sql = SqlBuilder;
```

**Methods (all `@overload("append")`):**

| Method | Input Type | SQL Output |
|--------|-----------|------------|
| `appendSafe` | `String` | Raw SQL (trusted) |
| `appendFragment` | `SqlFragment` | Nested fragment parts |
| `appendPart` | `SqlPart` | Single typed part |
| `appendPartList` | `List<SqlPart>` | Comma-separated parts |
| `appendBoolean` | `Boolean` | `TRUE` / `FALSE` |
| `appendDate` | `Date` | `'2024-12-25'` |
| `appendFloat64` | `Float64` | `19.99` |
| `appendInt32` | `Int32` | `42` |
| `appendInt64` | `Int64` | `43` |
| `appendString` | `String` | `'escaped''value'` |
| `append*List` | `Listed<T>` | Comma-separated of above |

**Result:** `.accumulated: SqlFragment`

### Escaping Rules

**Strings:** Single-quoted, with `'` doubled to `''`.
```
"Robert'); drop table hi;--" → 'Robert''); drop table hi;--'
```

**Integers/Floats:** Rendered as literals, no quoting.

**Booleans:** `TRUE` / `FALSE` (SQL keywords).

**Dates:** Single-quoted ISO 8601: `'2024-12-25'`.

**Lists:** Comma-separated, each element escaped per its type:
```
[1, 2, 3] → 1, 2, 3
["a", "b"] → 'a', 'b'
```

### Test Coverage

27 tests covering:
- Bobby Tables injection (`Robert'); drop table hi;--`)
- Empty strings, embedded quotes, Unicode, newlines
- All scalar types (Int32, Int64, Float64, Boolean, Date)
- All list types
- Fragment nesting and freezing
- Individual SqlPart composition

---

## 5. Compiler Integration: StringExprMacro

The Temper compiler (`temperlang/temper`) has a built-in macro system that
makes tagged strings work. The `StringExprMacro` in
`builtin/src/commonMain/kotlin/lang/temper/builtin/StringExprMacro.kt` (808 lines)
handles desugaring.

### How `sql"..."` Compiles

When the compiler sees:

```temper
let query = sql"SELECT * FROM users WHERE name = ${userName}";
```

It recognizes `sql` as a type (alias for `SqlBuilder`), which is an
Accumulator. The macro rewrites this to:

```temper
let query = do {
  let acc = new SqlBuilder();
  acc.appendSafe("SELECT * FROM users WHERE name = ");
  acc.append(userName);   // overload resolves based on userName's type
  acc.accumulated         // returns SqlFragment
};
```

### The Three Tag Conventions

1. **`null` tag (plain strings):** `"hello ${name}"` → string concatenation
2. **Type tag (Accumulator):** `sql"..."`, `html"..."` → allocate builder,
   call `appendSafe`/`append`, return `.accumulated`
3. **Function tag:** `tag"..."` where tag is a function → collect parts and
   interpolations into lists, pass to function

### Recent Compiler Evolution (Co-developing with secure-composition)

| Date | PR | What |
|------|-----|------|
| Jan 31 | #310 | Fix string tag functions |
| Jan 31 | #311 | TODO for adapting to accumulator style |
| Feb 2 | #308 | Rust backend: translate connected types |
| Feb 14 | — | `StringExprMacro supports tagged, complex strings` (PR #307) |
| Feb 14 | — | SAST transparency for contextual autoescapers (PR #332) |
| Feb 16 | #331 | Overload on generics |
| Feb 16 | #334 | Autoesc optimizations compatible with abstract collectors |
| Feb 18 | #342 | Fix autoescaper optimization (regex + doPure) |
| Feb 21 | #348 | StringExprMacro coalesces string chunks together |

**PR #307** is the foundational change: it teaches the compiler to handle
tagged strings where the tag is a type with `appendSafe`/`append`/`accumulated`.

**PR #332** adds SAST transparency: instead of picking escapers at runtime,
they're picked at compile time and inlined, so static analysis tools can
verify the escaping is correct.

**PR #348** optimizes by coalescing adjacent trusted string writes into single
`appendSafe` calls.

---

## 6. Related Repositories

### temperlang/tagged-string-composition-security-analysis

"Analysis of genAI tooling on ability to produce secure code and ability to
adapt to safe tagged string idioms." This is the research backing — empirical
evidence that LLMs can learn to use tagged string APIs correctly.

### temperlang/security-bug-examples

"Don't use this code" — intentionally vulnerable examples, likely used as
negative test cases for the security analysis.

### temperlang/temper (compiler)

The compiler itself, where `StringExprMacro` lives. The autoescaping
optimization passes are in the frontend stage. Recent work has been
co-evolving the compiler and secure-composition in lockstep.

---

## 7. Design Philosophy

From `config.temper.md`:

> The goal of this library is to allow developers to focus on building apps
> and systems and delegate securing the system against injection attacks to
> code designed and maintained by professional security engineers who focus
> on the nuances of secure composition.

**Key principles:**
1. **Type safety = system safety.** Safe types (`SafeHtml`, `SqlFragment`,
   `FilePath`) are only producible through the escaping pipeline.
2. **Contextual awareness.** The HTML escaper tracks 11 parsing states and
   delegates to nested language handlers for CSS, JS, and URLs.
3. **Compile-time optimization.** All operators are pure functions designed
   for inlining. The compiler's SAST transparency pass makes escaping
   decisions visible to static analysis.
4. **Cross-language.** Since Temper compiles to JS, Java, C#, Python, Rust,
   Lua — one security implementation serves all language communities.
5. **Streaming architecture.** State machines process chunks incrementally,
   no need to buffer entire output.

---

## 8. Implications for Skinny Ecto

### Current State

Skinny Ecto's `Query` builder operates on raw strings:
- `WhereClause` stores `field`, `operator`, `value` as plain strings
- `Record.data` is `Map<String, String>`
- No escaping anywhere — pure in-memory operation

### Integration Path

With secure-composition's SQL module, Skinny Ecto can produce `SqlFragment`
objects instead of raw strings. The query builder would generate SQL like:

```temper
let query = sql"SELECT ${columns} FROM ${tableName} WHERE ${conditions}";
```

Where interpolated values are automatically escaped per their type.

### What's Needed

1. **Temper compiler update:** Must be a version that supports tagged string
   macros (PR #307 and subsequent fixes — all merged as of Feb 21).

2. **Add secure-composition as a dependency:** Import the SQL module.

3. **Modify Query builder:** Instead of evaluating queries against an
   in-memory store directly, also (or instead) produce `SqlFragment` output
   that could be sent to a real database.

4. **Type the data model:** Move from `Map<String, String>` to typed fields
   so the SQL escaping can use the correct `SqlPart` subtype (Int32 vs String
   vs Date).

### Value Proposition

This transforms Skinny Ecto from a teaching toy into a demonstration of
**injection-proof query building by construction** — the exact security
property that Ecto provides in Elixir through its parameterized queries,
but achieved here through Temper's type system and compile-time macro
expansion.
