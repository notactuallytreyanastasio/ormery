# ORMery - Makefile
# A simplified Ecto implementation in Temper
#
# Usage: make help

.PHONY: help build build-js build-py bundle test run serve open \
        clean clean-build clean-bundle \
        graph graph-serve graph-sync graph-check \
        check deps setup all

# Default port for tutorial server
PORT ?= 8000

# Temper compiler (override with TEMPER=... if not on PATH)
TEMPER ?= temper

# Colors for output
CYAN  := \033[36m
GREEN := \033[32m
RESET := \033[0m

##@ General
help: ## Show this help message
	@echo "ORMery - Simplified Ecto in Temper"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"; printf "Usage: make $(CYAN)<target>$(RESET)\n\n"} \
		/^##@/ { printf "\n$(GREEN)%s$(RESET)\n", substr($$0, 5) } \
		/^[a-zA-Z_-]+:.*?##/ { printf "  $(CYAN)%-18s$(RESET) %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

##@ Build
build: build-js ## Build all targets (alias for build-js)

build-js: ## Compile Temper source to JavaScript
	@echo "Compiling Temper to JavaScript..."
	$(TEMPER) build --backend js
	@echo "Build complete: temper.out/js/"

build-py: ## Compile Temper source to Python
	@echo "Compiling Temper to Python..."
	$(TEMPER) build --backend py
	@echo "Build complete: temper.out/py/"

bundle: build-js ## Build production tutorial bundle with Vite
	@echo "Bundling tutorial with Vite..."
	npx vite build
	@echo "Production bundle ready in dist/"

##@ Run
run: build-js ## Run the demo via Node.js
	node temper.out/js/ormery/ormery.js

run-py: build-py ## Run the demo via Python
	./run-python.py

test: run-py ## Run tests (Python backend)

##@ Tutorial & Demo
serve: build-js ## Start Vite dev server for tutorial (PORT=8000)
	@echo "Starting Vite dev server..."
	@echo "  Static Tutorial:       http://localhost:$(PORT)/"
	@echo "  Interactive Playground: http://localhost:$(PORT)/interactive.html"
	@echo "  Temper Demo:           http://localhost:$(PORT)/demo.html"
	npx vite --port $(PORT)

open: ## Open the interactive playground in your browser
	@open http://localhost:$(PORT)/interactive.html 2>/dev/null || \
		xdg-open http://localhost:$(PORT)/interactive.html 2>/dev/null || \
		echo "Open http://localhost:$(PORT)/interactive.html in your browser"

open-tutorial: ## Open the static tutorial in your browser
	@open http://localhost:$(PORT)/ 2>/dev/null || \
		xdg-open http://localhost:$(PORT)/ 2>/dev/null || \
		echo "Open http://localhost:$(PORT)/ in your browser"

open-demo: ## Open the Temper-compiled demo in your browser
	@open http://localhost:$(PORT)/demo.html 2>/dev/null || \
		xdg-open http://localhost:$(PORT)/demo.html 2>/dev/null || \
		echo "Open http://localhost:$(PORT)/demo.html in your browser"

##@ Decision Graph
graph: ## View decision graph nodes and edges
	@deciduous nodes
	@echo ""
	@deciduous edges

graph-serve: ## Start the decision graph viewer
	deciduous serve

graph-sync: ## Export decision graph for GitHub Pages
	deciduous sync
	@echo "Exported to docs/graph-data.json and docs/git-history.json"

graph-check: ## Audit decision graph for orphaned nodes
	@echo "Checking decision graph health..."
	@deciduous nodes
	@echo ""
	@deciduous edges
	@echo ""
	@echo "Review: every outcome should link to an action/goal."
	@echo "        every action should link to a goal/decision."

##@ Cleanup
clean: clean-build clean-bundle ## Remove all build artifacts

clean-build: ## Remove Temper compilation output
	rm -rf temper.out/
	@echo "Removed temper.out/"

clean-bundle: ## Remove Vite build output
	rm -rf dist/
	@echo "Removed dist/"

##@ Checks
setup: ## Install npm dependencies (Vite)
	npm install

deps: ## Check that required tools are installed
	@echo "Checking dependencies..."
	@command -v $(TEMPER) >/dev/null 2>&1 && echo "  $(TEMPER): OK" || echo "  $(TEMPER): MISSING - install from https://github.com/temperlang/temper"
	@command -v node >/dev/null 2>&1 && echo "  node: OK ($(shell node --version 2>/dev/null))" || echo "  node: MISSING"
	@command -v npx >/dev/null 2>&1 && echo "  npx: OK" || echo "  npx: MISSING"
	@test -d node_modules && echo "  node_modules: OK" || echo "  node_modules: MISSING - run 'make setup'"
	@command -v deciduous >/dev/null 2>&1 && echo "  deciduous: OK" || echo "  deciduous: MISSING (optional - for decision graph)"
	@command -v gh >/dev/null 2>&1 && echo "  gh: OK" || echo "  gh: MISSING (optional - for GitHub CLI)"

check: deps ## Verify project health (deps + git status)
	@echo ""
	@echo "Git status:"
	@git status --short
	@echo ""
	@echo "Source files:"
	@ls -1 src/*.temper.md
	@echo ""
	@echo "Tutorial files:"
	@ls -1 tutorial/*.html

##@ All-in-One
all: build bundle ## Build everything and bundle for production
	@echo ""
	@echo "Ready! Run 'make serve' to start the Vite dev server."

startup: setup deps build ## Full startup: install deps, check tools, build
	@echo ""
	@echo "Startup complete. Available commands:"
	@echo "  make serve   - Start Vite dev server"
	@echo "  make open    - Open interactive playground"
	@echo "  make run     - Run CLI demo"
	@echo "  make graph   - View decision graph"
