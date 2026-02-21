# Skinny Ecto - Project Instructions

## Overview

A simplified Ecto implementation in Temper demonstrating query builders, schemas, and in-memory data stores.

## Project Structure

- `src/` - Temper source files (`.temper.md` literate programming format)
- `tutorial/` - Three tutorial sites (static, interactive, real Temper demo)
- `docs/` - Decision graph exports for GitHub Pages
- `.deciduous/` - Decision tracking database
- `.claude/` - Claude Code hooks and custom commands

## Build & Test

```bash
# Compile Temper to JavaScript
temper build --backend js

# Run demo
node temper.out/skinny-ecto.js

# Run tests
./run-python.py

# Serve tutorials
cd tutorial && python3 -m http.server 8000
```

## Decision Graph Workflow

**THIS IS MANDATORY. Log decisions IN REAL-TIME.**

### Available Commands

| Command | Purpose |
|---------|---------|
| `/decision` | Manage decision graph |
| `/recover` | Recover context on session start |
| `/work` | Start work transaction |
| `/build-test` | Build and test |
| `/serve-ui` | Start decision graph viewer |
| `/sync-graph` | Export to GitHub Pages |

### Core Rule

```
BEFORE implementation -> Log what you're about to do
AFTER outcome -> Log the result
CONNECT immediately -> Link every node to its parent
```

### Session Start Checklist

```bash
deciduous nodes           # Review existing decisions
deciduous edges           # Check connections
git status                # Current state
```

### Quick Commands

```bash
deciduous add goal "Title" -c 90 -p "User request"
deciduous add action "Title" -c 85
deciduous link FROM TO -r "reason"
deciduous serve          # View graph locally
deciduous sync           # Export for GitHub Pages
```

## Git Workflow

**ALWAYS stage files explicitly:**
- ✅ `git add src/skinny-ecto.temper.md`
- ✅ `git add tutorial/index.html`
- ❌ `git add .` or `git add -A` (NEVER use these)

**Link commits to decision graph:**
```bash
git commit -m "feat: add query builder"
deciduous add action "Implemented query builder" -c 90 --commit HEAD
deciduous link <goal_id> <action_id> -r "Implementation"
```

## Code Style

- Use literate programming format (`.temper.md`)
- Follow Temper naming conventions
- Document all public APIs
- Include examples in markdown sections
- Test all query operations

## Testing Strategy

- Unit tests for individual components
- Integration tests for query pipeline
- Edge cases: empty results, invalid fields, type mismatches
- Performance tests for large datasets

## Deployment

### GitHub Pages (Decision Graph)

```bash
deciduous sync           # Export graph + git history
git add docs/graph-data.json docs/git-history.json
git commit -m "docs: update decision graph"
git push
```

Enable Pages: Settings > Pages > Deploy from /docs folder

### Tutorial Sites

Tutorial sites are static HTML and can be deployed anywhere:
- GitHub Pages
- Netlify
- Vercel
- Any static hosting

## Development Focus

This is a **demonstration project** showcasing:
1. Temper language features
2. Type-safe query building
3. Functional programming patterns
4. Decision tracking with Deciduous

Keep implementations **simple and educational** rather than production-ready.
