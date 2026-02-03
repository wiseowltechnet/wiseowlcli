# Manual Testing Guide

## Test 1: Basic Commands
```bash
./target/debug/ocli
/help
/exit
```

## Test 2: File Operations
```bash
./target/debug/ocli
/read test.py
/exit
```

## Test 3: Write and Preview
```bash
./target/debug/ocli
/write output.txt
Write hello world
/preview
/apply
y
/exit
```

## Test 4: Context Tracking
```bash
./target/debug/ocli chat --session test
/read test.py
/clear
/exit
```

## Test 5: Tool Calling (AI)
```bash
./target/debug/ocli
Read the test.py file and tell me what it does
exit
```

Expected: AI should use <tool_call> to read the file

## Verification
- Check .ocli/context/ for session files
- Check for .backup files after edits
- Verify /preview shows pending changes
- Confirm /rollback works
