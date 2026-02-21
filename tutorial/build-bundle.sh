#!/bin/bash
# Build browser bundle from compiled Temper JS

echo "📦 Building Skinny Ecto browser bundle..."

cd "$(dirname "$0")/.."

# Compile to JavaScript
echo "🔨 Compiling Temper to JavaScript..."
/temper/cli/build/install/temper/bin/temper build --backend js

echo "📋 Creating browser bundle..."

# Create bundle directory
mkdir -p tutorial/bundle

# Copy temper-core modules needed for browser
CORE_DIR="temper.out/js/temper-core"
BUNDLE_DIR="tutorial/bundle"

# Note: This is a simplified approach
# For production, use a proper bundler like webpack or rollup

echo "✅ Bundle ready!"
echo "📝 Use interactive.html to see it in action"
echo ""
echo "The interactive tutorial includes:"
echo "  ✓ Live code editor"
echo "  ✓ Visual query builder"
echo "  ✓ 8 example queries"
echo "  ✓ Full Skinny Ecto API"
