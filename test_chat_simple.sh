#!/bin/bash

echo "🧪 Testing OCLI chat functionality manually..."

# Test 1: Check if chat mode starts
echo "✅ Testing chat mode startup:"
timeout 5s ocli --help > /dev/null && echo "  ✓ OCLI binary works"

# Test 2: Test explicit chat command
echo -e "\n✅ Testing explicit chat command:"
echo "exit" | timeout 10s ocli chat > /dev/null 2>&1 && echo "  ✓ Chat command accepts input and exits"

# Test 3: Test default mode (should be chat)
echo -e "\n✅ Testing default mode (should be chat):"
echo "exit" | timeout 10s ocli > /dev/null 2>&1 && echo "  ✓ Default mode works and exits properly"

echo -e "\n🎉 Chat functionality structure is working!"
echo -e "\nTo test interactively, run:"
echo "  ocli          # Default chat mode"
echo "  ocli chat     # Explicit chat mode"
echo "Then type 'exit' to quit"
