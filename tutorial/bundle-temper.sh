#!/bin/bash
# Bundle Temper-compiled JS for browser use

echo "📦 Creating browser bundle from Temper output..."

cd "$(dirname "$0")/.."

# Ensure we have compiled JS
echo "🔨 Compiling Temper to JavaScript..."
/temper/cli/build/install/temper/bin/temper build --backend js

# Copy compiled files to tutorial
echo "📋 Copying compiled modules..."
mkdir -p tutorial/lib

# Copy temper-core
cp -r temper.out/js/temper-core tutorial/lib/

# Copy skinny-ecto
cp -r temper.out/js/skinny-ecto tutorial/lib/

echo "✅ Bundle created in tutorial/lib/"
echo ""
echo "Files:"
ls -lh tutorial/lib/
echo ""
echo "📝 Now you can use:"
echo "   import { Field, Schema, Query } from './lib/skinny-ecto/skinny_ecto.js'"
