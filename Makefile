# Skinny Ecto - Makefile
# A simplified Ecto implementation in Temper
#
# Usage: make help

.PHONY: help build build-js build-py bundle test run serve open \
        clean clean-build clean-bundle \
        graph graph-serve graph-sync graph-check \
        check deps all

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
	@echo "Skinny Ecto - Simplified Ecto in Temper"
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

bundle: build-js ## Bundle compiled JS into tutorial/lib for browser use
	@echo "Bundling compiled JS for browser..."
	@mkdir -p tutorial/lib
	cp -r temper.out/js/temper-core tutorial/lib/
	cp -r temper.out/js/skinny-ecto tutorial/lib/
	@echo "Bundle ready in tutorial/lib/"

##@ Run
run: build-js ## Run the demo via Node.js
	node temper.out/js/skinny-ecto/skinny_ecto.js

run-py: build-py ## Run the demo via Python
	./run-python.py

test: run-py ## Run tests (Python backend)

##@ Tutorial & Demo
serve: ## Start tutorial server on localhost (PORT=8000)
	@echo "Starting tutorial server on http://localhost:$(PORT)"
	@echo "  Static Tutorial:       http://localhost:$(PORT)/"
	@echo "  Interactive Playground: http://localhost:$(PORT)/interactive.html"
	@echo "  Temper Demo:           http://localhost:$(PORT)/demo.html"
	@echo ""
	@echo "Press Ctrl+C to stop"
	@cd tutorial && python3 -m http.server $(PORT)

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

clean-bundle: ## Remove bundled JS from tutorial/lib
	rm -rf tutorial/lib/temper-core tutorial/lib/skinny-ecto
	rm -rf tutorial/bundle
	@echo "Removed tutorial/lib/ bundles"

##@ Checks
deps: ## Check that required tools are installed
	@echo "Checking dependencies..."
	@command -v $(TEMPER) >/dev/null 2>&1 && echo "  $(TEMPER): OK" || echo "  $(TEMPER): MISSING - install from https://github.com/temperlang/temper"
	@command -v node >/dev/null 2>&1 && echo "  node: OK ($(shell node --version 2>/dev/null))" || echo "  node: MISSING"
	@command -v python3 >/dev/null 2>&1 && echo "  python3: OK ($(shell python3 --version 2>/dev/null))" || echo "  python3: MISSING"
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
all: build bundle ## Build everything and prepare tutorial bundle
	@echo ""
	@echo "Ready! Run 'make serve' to start the tutorial server."

startup: deps build bundle ## Full startup: check deps, build, bundle
	@echo ""
	@echo "Startup complete. Available commands:"
	@echo "  make serve   - Start tutorial server"
	@echo "  make open    - Open interactive playground"
	@echo "  make run     - Run CLI demo"
	@echo "  make graph   - View decision graph"
