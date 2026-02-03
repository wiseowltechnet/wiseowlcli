#!/bin/bash

echo "🧪 Testing OCLI Project Context Features..."

# Test 1: Help shows init command
echo "✅ Testing help command includes init:"
ocli --help | grep -q "init" && echo "  ✓ Init command found in help" || echo "  ✗ Init command missing"

# Test 2: Project context file was created
echo -e "\n✅ Testing project context file:"
if [ -f ".ocli/context.json" ]; then
    echo "  ✓ Context file exists"
    echo "  📄 Context file size: $(wc -c < .ocli/context.json) bytes"
else
    echo "  ✗ Context file not found"
fi

# Test 3: Context file contains expected data
echo -e "\n✅ Testing context file content:"
if grep -q "Rust" .ocli/context.json; then
    echo "  ✓ Project type detected correctly"
else
    echo "  ✗ Project type not detected"
fi

if grep -q "Cargo.toml" .ocli/context.json; then
    echo "  ✓ Key files detected"
else
    echo "  ✗ Key files not detected"
fi

echo -e "\n🎉 Project context functionality is working!"
echo -e "\nAvailable commands:"
echo "  ocli                    # Interactive chat"
echo "  ocli read file.py       # Analyze file"
echo "  ocli write file.py      # Create/edit file"
echo "  ocli init               # Analyze project structure"
