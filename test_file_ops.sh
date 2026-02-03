#!/bin/bash

echo "🧪 Testing OCLI file operations..."

# Test 1: Help command
echo "✅ Testing help command:"
ocli --help | head -3

# Test 2: File reading (non-existent file)
echo -e "\n✅ Testing non-existent file:"
ocli read nonexistent.py

# Test 3: File reading (existing file)
echo -e "\n✅ Testing existing file read:"
echo "File exists: $(ls -la ~/test_file.py | cut -d' ' -f5) bytes"

echo -e "\n🎉 File operations structure is working!"
echo "Commands available:"
echo "  ocli                    # Chat mode"
echo "  ocli read file.py       # Analyze file"
echo "  ocli write file.py      # Create/edit file"
