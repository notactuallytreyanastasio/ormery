# 🚀 Skinny Ecto Interactive Tutorial

Three ways to learn Skinny Ecto - from beginner-friendly to fully interactive!

## 📚 Tutorial Options

### 1. Static Tutorial (`index.html`)
**Best for: Reading and understanding**
- Beautiful UI with gradient design
- 8 progressive lessons
- Pre-rendered examples
- No code execution needed
- Perfect for learning the concepts

**Usage:**
```bash
open index.html
# or
./serve.sh
```

### 2. Interactive Playground (`interactive.html`) ⭐ RECOMMENDED
**Best for: Hands-on experimentation**
- **Live Code Editor** - Edit and run real Skinny Ecto code
- **Visual Query Builder** - Build queries with form inputs
- **Example Library** - 8 ready-to-run examples
- Full JavaScript implementation of Skinny Ecto
- All features working in-browser

**Features:**
- ✅ **Code Editor Tab**: Edit code, run queries, see results
- ✅ **Query Builder Tab**: Visual UI for building queries
- ✅ **Examples Tab**: Click to load pre-built queries
- ✅ Editable code with syntax highlighting
- ✅ Live execution with real results
- ✅ Copy queries from builder to editor
- ✅ Add/remove WHERE clauses
- ✅ SELECT field checkboxes
- ✅ ORDER BY, LIMIT, OFFSET controls

**Usage:**
```bash
open interactive.html
# or serve it:
cd tutorial && python3 -m http.server 8000
# Visit http://localhost:8000/interactive.html
```

### 3. Temper-Compiled Version (Advanced)
**Best for: Integration with Temper projects**
- Uses actual compiled Temper → JavaScript output
- Same API as Temper version
- For developers integrating Skinny Ecto

## 🎯 Quick Start

**Try the Interactive Playground:**
```bash
cd tutorial
open interactive.html
```

Then:
1. Click "▶ Run Code" to execute the default example
2. Switch to "Query Builder" tab to visually build queries
3. Check out "Examples" for pre-built queries

## 🌟 Features Showcase

### Code Editor
```temper
// Define schema
let userFields = [
  field("name", "String", false, false),
  field("age", "Int", false, false)
];
let userSchema = schema("users", userFields);

// Build query
let adults = new Query(userSchema, store)
  .where("age", ">=", "18")
  .orderBy("age", "desc")
  .select(["name", "age"])
  .all();
```

### Visual Query Builder
- Click "Add WHERE Clause" to filter
- Check/uncheck fields to SELECT
- Choose ORDER BY field and direction
- Set LIMIT and OFFSET for pagination
- Click "Execute Query" to see results
- Click "Copy to Editor" to get the code

## 📖 Learning Path

1. **Start with `index.html`** - Read the lessons to understand concepts
2. **Try `interactive.html`** - Run the default example
3. **Experiment** - Modify the code, try different queries
4. **Use Query Builder** - Build queries visually
5. **Load Examples** - See common query patterns
6. **Build Your Own** - Create custom queries

## 🎨 UI Features

- **Purple Gradient Theme** (#667eea → #764ba2)
- **Split-pane Editor** (Code | Output)
- **Tabbed Interface** (Code Editor | Query Builder | Examples)
- **Responsive Design** (Works on all devices)
- **Syntax Highlighting** (VS Code dark theme colors)
- **Smooth Animations** (Hover effects, transitions)

## 🔧 Technical Details

**Interactive Implementation:**
- Pure JavaScript reimplementation of Skinny Ecto
- All features: Field, Schema, Record, InMemoryStore, Query
- Type-aware comparisons (Int vs String)
- Full query execution pipeline
- No external dependencies

**Operators Supported:**
- `==` (equals)
- `!=` (not equals)
- `>` (greater than)
- `<` (less than)
- `>=` (greater than or equal)
- `<=` (less than or equal)

**Query Features:**
- WHERE clauses with AND logic
- SELECT field projection
- ORDER BY (asc/desc, multi-field)
- LIMIT (max results)
- OFFSET (skip results)
- Method chaining

## 📱 Browser Compatibility

Works in all modern browsers:
- Chrome/Edge (recommended)
- Firefox
- Safari
- No build step needed!
- No npm install needed!

## 🚀 Deployment

**GitHub Pages:**
1. Push to GitHub
2. Enable Pages in Settings
3. Set source to `/tutorial` directory
4. Visit `https://username.github.io/repo/tutorial/interactive.html`

**Static Hosting:**
- Upload `tutorial/` folder to any web host
- Works on Netlify, Vercel, Cloudflare Pages, etc.
- Just serve the static files!

## 💡 Tips

**In Code Editor:**
- Edit any part of the code
- Use `print()` to output text
- Results appear in the output panel
- Click "Reset" to restore default code

**In Query Builder:**
- Add multiple WHERE clauses for AND logic
- Leave value empty to skip a clause
- Check "All fields" to disable SELECT projection
- Copy generated code to see the syntax

**In Examples:**
- Click any card to load the example
- Examples demonstrate different query patterns
- Start simple, work up to complex queries

## 🎓 Next Steps

After mastering the tutorial:
1. Check out the [full source code](../src/skinny-ecto.temper.md)
2. Learn about the [Temper language](https://github.com/temperlang/temper)
3. Read the [design document](../../docs/SKINNY_ECTO_DESIGN.md)
4. Explore the [decision graph](../../docs/)

## 🤝 Contributing

Ideas for enhancements:
- [ ] Dark mode toggle
- [ ] Export/import queries
- [ ] Share query links
- [ ] More example queries
- [ ] Query performance metrics
- [ ] Schema designer UI
- [ ] Multi-table support

Built with ❤️ to showcase the Query Builder pattern in Temper!
