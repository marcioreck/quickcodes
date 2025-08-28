#!/bin/bash

# QuickCodes WASM Test Server
# Simple HTTP server to test the WebAssembly examples

PORT=${1:-8000}

echo "🚀 Starting QuickCodes WASM Test Server..."
echo "📂 Serving from: $(pwd)"
echo "🌐 URL: http://localhost:$PORT"
echo ""
echo "📋 Available demos:"
echo "   📝 Generator: http://localhost:$PORT/examples/generator.html"
echo "   📷 Reader:    http://localhost:$PORT/examples/reader.html"
echo ""
echo "⏹️  Press Ctrl+C to stop"
echo ""

# Try different Python versions
if command -v python3 &> /dev/null; then
    python3 -m http.server $PORT
elif command -v python &> /dev/null; then
    python -m http.server $PORT
else
    echo "❌ Python not found. Please install Python or use another web server."
    echo "💡 Alternative: npx serve . --port $PORT"
    exit 1
fi
