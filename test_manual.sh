#!/bin/bash

echo "🧪 Testing OCLI manually..."

# Test 1: Check if binary exists and is executable
if [ -x ~/.local/bin/ocli ]; then
    echo "✅ OCLI binary is installed and executable"
else
    echo "❌ OCLI binary not found or not executable"
    exit 1
fi

# Test 2: Check help output
if ~/.local/bin/ocli --help | grep -q "Ollama CLI"; then
    echo "✅ Help output looks correct"
else
    echo "❌ Help output issue"
fi

# Test 3: Check if Ollama is accessible
if curl -s http://localhost:11434/api/tags > /dev/null; then
    echo "✅ Ollama is running and accessible"
else
    echo "❌ Ollama is not running"
    exit 1
fi

echo "🎉 Manual tests passed! OCLI is ready to use."
echo "Run 'ocli' to start chatting!"
