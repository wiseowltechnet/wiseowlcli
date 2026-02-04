# WiseOwl CLI Tests

## Test Framework

Uses `expect` for interactive conversation testing with the CLI.

## Running Tests

```bash
# Run all tests
./tests/run_tests.sh

# Run specific test
./tests/conversation_test.exp
```

## Test Coverage

### Conversation Test (`conversation_test.exp`)
- ✅ CLI startup
- ✅ Simple conversation (What is 2+2?)
- ✅ Progress indicator display
- ✅ AI response
- ✅ Dashboard command
- ✅ Direct file commands
- ✅ Clean exit

## Test Structure

```expect
#!/usr/bin/expect -f

set timeout 30
spawn ./target/release/wiseowlcli

# Test pattern
expect "pattern" {
    puts "✅ Test passed"
}

send "command\r"
expect $prompt
```

## Adding New Tests

1. Create new `.exp` file in `tests/`
2. Add to `run_tests.sh`
3. Follow existing test patterns

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

## Known Limitations

- Requires interactive terminal
- Depends on Ollama response time
- May timeout on slow systems
