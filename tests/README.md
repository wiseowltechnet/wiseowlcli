# WiseOwl CLI Tests

## Test Framework

Uses `expect` for interactive conversation testing with the CLI.

## Test Suites

### 1. Conversation Test (`conversation_test.exp`)
Basic functionality testing:
- ✅ CLI startup
- ✅ Simple conversation
- ✅ Progress indicator
- ✅ Dashboard command
- ✅ Direct file commands

### 2. Developer Workflow Test (`developer_workflow_test.exp`)
Comprehensive project creation test:
- ✅ REST API (Go)
- ✅ CLI Tool (Rust)
- ✅ Web Server (Node.js)
- ✅ Database Schema (PostgreSQL)
- ✅ Dockerfile
- ✅ Test Suite (Python)
- ✅ Config File (YAML)
- ✅ Makefile
- ✅ GitHub Actions
- ✅ README

### 3. Quick Dev Test (`quick_dev_test.exp`)
Fast validation of AI usefulness:
- Tests 10 different project types
- Verifies AI responses
- Checks dashboard stats
- Pass criteria: 8/10 projects created

## Running Tests

```bash
# Run all tests
./tests/run_tests.sh

# Run specific test
./tests/conversation_test.exp
./tests/developer_workflow_test.exp
./tests/quick_dev_test.exp
```

## Test Structure

```expect
#!/usr/bin/expect -f

set timeout 45
set prompt "You: "

spawn ./target/release/wiseowlcli

expect "WISEOWL CLI"
expect $prompt

# Test pattern
send "Create something\r"
expect "AI:" {
    puts "✅ AI responded"
}
expect $prompt
```

## Success Criteria

### Conversation Test
- All basic commands work
- Dashboard opens
- Clean exit

### Developer Workflow Test
- AI creates 10 different project types
- Files are generated
- Dashboard shows stats

### Quick Dev Test
- At least 8/10 projects created successfully
- AI responses are useful
- Performance metrics tracked

## Adding New Tests

1. Create new `.exp` file in `tests/`
2. Follow existing test patterns
3. Add to `run_tests.sh`
4. Document in this README

## Requirements

- `expect` package: `sudo apt install expect`
- Built binary: `cargo build --release`
- Running Ollama instance

## CI/CD Integration

```yaml
# .github/workflows/test.yml
- name: Run tests
  run: |
    cargo build --release
    ./tests/run_tests.sh
```

## Test Philosophy

**TDD First** - Tests define expected behavior
- Write test before feature
- Test actual user workflows
- Verify AI usefulness
- Catch regressions early

## Known Limitations

- Requires interactive terminal
- Depends on Ollama response time
- May timeout on slow systems
- AI responses vary (non-deterministic)

## Troubleshooting

**Test timeouts:**
- Increase timeout value
- Check Ollama is running
- Verify model is loaded

**AI not responding:**
- Check Ollama logs
- Verify model availability
- Test with simpler prompts

**Dashboard not opening:**
- Check terminal size (>= 80x24)
- Verify ratatui dependencies
- Test with `/dashboard` manually
