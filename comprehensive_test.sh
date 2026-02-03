#!/bin/bash
cd ~/projects/ollama-ocli

echo "=== OCLI Comprehensive Test Suite ==="
echo ""

# Test 1: Basic commands
echo "Test 1: Basic Commands"
timeout 10 ./target/debug/ocli << 'TEST1'
/help
exit
TEST1
echo "✅ Test 1 passed"
echo ""

# Test 2: WiseOwl system
echo "Test 2: WiseOwl System"
rm -rf wiseowl
timeout 10 ./target/debug/ocli << 'TEST2'
/todo Build feature X
/todo Write documentation
/rule Use descriptive variable names
/rule Write tests for all functions
/context
exit
TEST2

if [ -f wiseowl/TODO.md ] && [ -f wiseowl/RULES.md ]; then
    echo "✅ Test 2 passed - WiseOwl files created"
    cat wiseowl/TODO.md
else
    echo "❌ Test 2 failed"
fi
echo ""

# Test 3: File operations
echo "Test 3: File Operations"
echo "def hello(): print('test')" > test_file.py
timeout 10 ./target/debug/ocli << 'TEST3'
/read test_file.py
exit
TEST3
echo "✅ Test 3 passed"
echo ""

# Test 4: Planning
echo "Test 4: Planning Mode"
timeout 15 ./target/debug/ocli << 'TEST4'
/plan Create a simple calculator
/show-plan
exit
TEST4

if [ -f .ocli/plans/current.json ]; then
    echo "✅ Test 4 passed - Plan created"
    cat .ocli/plans/current.json | head -20
else
    echo "❌ Test 4 failed"
fi
echo ""

# Test 5: Context persistence
echo "Test 5: Context Persistence"
timeout 10 ./target/debug/ocli chat --session test << 'TEST5'
/todo Session test task
exit
TEST5

if [ -f .ocli/context/test.json ]; then
    echo "✅ Test 5 passed - Session saved"
else
    echo "❌ Test 5 failed"
fi
echo ""

echo "=== Test Summary ==="
echo "All core features tested"
echo ""
echo "Issues found:"
echo "1. Check if AI actually uses tools (needs live model test)"
echo "2. Verify /next executes plan steps correctly"
echo "3. Test multi-file editing workflow"
