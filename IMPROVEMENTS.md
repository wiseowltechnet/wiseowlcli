# OCLI Improvements Needed

## Issues Found

### 1. Planning Command Slow
- /plan calls AI which takes 30+ seconds
- Need: Fast plan templates or async generation
- Fix: Add template-based planning

### 2. Tool Calling Not Verified
- System prompt teaches AI about tools
- But no live test of AI actually using them
- Need: Test with actual AI conversation

### 3. Multi-File Editing Workflow
- /write adds to pending
- /preview shows changes
- /apply saves
- Need: Test full workflow with multiple files

### 4. Error Handling
- Some commands fail silently
- Need: Better error messages and recovery

## Proposed Fixes

### Quick Wins
1. Add /plan-quick with templates
2. Add /test-tools command to verify tool calling
3. Improve error messages
4. Add progress indicators for slow operations

### Medium Term
1. Async plan generation (don't block)
2. Plan templates library
3. Better diff visualization
4. Undo/redo stack

### Long Term
1. LSP integration for code intelligence
2. Git integration (auto-commit, PR gen)
3. Syntax highlighting
4. Plugin system
