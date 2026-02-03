#!/bin/bash

cd ~/projects/ollama-ocli

echo "Testing OCLI with tool calling..."
echo ""

# Create test file
echo "print('hello world')" > test_file.py

# Test with expect
expect << 'EXPECT_EOF'
set timeout 30
spawn ./target/debug/ocli

expect "You: "
send "/read test_file.py\r"

expect "You: "
send "What's in my working files?\r"

expect "You: "
send "/write output.txt\r"

expect "You: "
send "Write 'Testing OCLI tools' to this file\r"

expect "You: "
send "/preview\r"

expect "You: "
send "exit\r"

expect eof
EXPECT_EOF

echo ""
echo "Test complete!"
ls -la test_file.py output.txt 2>/dev/null || echo "Files not found"
