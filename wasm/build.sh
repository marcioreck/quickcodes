#!/bin/bash

# QuickCodes WebAssembly Build Script
# Builds the WASM module for browser and Node.js usage

set -e

echo "🚀 Building QuickCodes WebAssembly module..."

# Ensure wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build for web (browser)
echo "📦 Building for web browsers..."
wasm-pack build --target web --out-dir pkg/web --release

# Build for Node.js
echo "📦 Building for Node.js..."
wasm-pack build --target nodejs --out-dir pkg/nodejs --release

# Build for bundlers (webpack, rollup, etc.)
echo "📦 Building for bundlers..."
wasm-pack build --target bundler --out-dir pkg/bundler --release

echo "✅ WebAssembly builds completed successfully!"
echo ""
echo "📁 Generated packages:"
echo "   - pkg/web/     - For direct browser usage"
echo "   - pkg/nodejs/  - For Node.js usage"
echo "   - pkg/bundler/ - For webpack, rollup, vite, etc."
echo ""
echo "🎯 Next steps:"
echo "   1. Test the builds with the example HTML files"
echo "   2. Publish to npm with: npm publish pkg/web"
