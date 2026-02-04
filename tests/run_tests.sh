#!/bin/bash
# WiseOwl CLI Test Suite

echo "🧪 WiseOwl CLI Test Suite"
echo "=========================="
echo ""

# Check if binary exists
if [ ! -f target/release/wiseowlcli ]; then
    echo "❌ Binary not found. Run: cargo build --release"
    exit 1
fi

# Check if expect is installed
if ! command -v expect &> /dev/null; then
    echo "❌ expect not installed. Run: sudo apt install expect"
    exit 1
fi

# Run conversation test
echo "Running conversation tests..."
echo ""

if ./tests/conversation_test.exp; then
    echo ""
    echo "✅ All tests passed!"
    exit 0
else
    echo ""
    echo "❌ Tests failed!"
    exit 1
fi
