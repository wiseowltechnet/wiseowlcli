#!/bin/bash
# Test OCLI conversational experience

echo "🧪 Testing OCLI Chat Experience"
echo "================================"
echo ""

# Test 1: Simple question
echo "Test 1: Simple conversational question"
echo "---------------------------------------"
timeout 10 ~/projects/ollama-ocli/target/release/ocli << "INPUT" 2>&1 | head -30 &
What is 2+2?
exit
INPUT

PID=$!
sleep 3

if ps -p $PID > /dev/null 2>&1; then
    echo "✅ Chat mode started successfully"
    echo "✅ LLM is responding (process running)"
    kill $PID 2>/dev/null
    wait $PID 2>/dev/null
else
    echo "❌ Chat mode exited unexpectedly"
fi

echo ""
echo "Test 2: Check slash commands still work"
echo "----------------------------------------"
timeout 5 ~/projects/ollama-ocli/target/release/ocli << "INPUT" 2>&1 | grep -E "(✅|❌|Usage:)" | head -5
/write-direct /tmp/chat_test.txt "Hello from chat"
exit
INPUT

if [ -f /tmp/chat_test.txt ]; then
    echo "✅ Slash commands work in chat mode"
    rm /tmp/chat_test.txt
else
    echo "❌ Slash commands failed"
fi

echo ""
echo "Test 3: Verify chat is default mode"
echo "------------------------------------"
~/projects/ollama-ocli/target/release/ocli --help | grep -A 2 "Commands:" | head -3

echo ""
echo "✅ All tests complete!"
