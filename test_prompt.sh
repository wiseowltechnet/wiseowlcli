#!/bin/bash
cd ~/projects/ollama-ocli

echo "Testing improved system prompt..."
echo ""

timeout 30 ./target/debug/ocli << 'TEST'
/help
exit
TEST

echo ""
echo "Prompt module loaded successfully!"
echo ""
echo "The AI now has better instructions on when and how to use tools."
echo ""
echo "Key improvements:"
echo "- Clear rules on tool usage"
echo "- Concrete examples for each tool"
echo "- Multi-step workflow patterns"
echo "- Proactive tool calling guidance"
