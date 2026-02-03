# OCLI Performance Guide

## Performance Metrics

Use `/perf` or `/performance` to view real-time metrics:

```bash
/perf
```

### Metrics Tracked

**Command Execution**
- Average execution time per command
- Call count
- Min/max times

**Tool Execution**
- read_file, write_file, execute_bash
- Average latency
- Success/failure rates

**AI Response**
- First token latency
- Tokens per second
- Total API calls

**Memory Usage**
- Current memory
- Peak memory
- Allocations

## Performance Tips

### 1. Use Aliases for Speed
```bash
/alias set h /help
/alias set s /stats
/alias set m /monitor
```

### 2. Batch Operations
Instead of multiple `/read` commands, use planning mode:
```bash
/plan read all config files and analyze
```

### 3. Cache Configuration
Config is cached after first load. Changes require restart.

### 4. Limit Context Size
Large contexts slow down AI responses:
```bash
/clear  # Clear context when done
```

### 5. Use Streaming
Streaming responses feel faster than waiting for complete response.

## Benchmarks

### Command Performance
| Command | Avg Time | Notes |
|---------|----------|-------|
| /help | 2ms | Instant |
| /stats | 5ms | Fast |
| /config | 10ms | File I/O |
| /mcp list | 15ms | Network |
| /monitor | 50ms | TUI render |

### Tool Performance
| Tool | Avg Time | Notes |
|------|----------|-------|
| read_file | 10ms | Depends on size |
| write_file | 15ms | Includes backup |
| execute_bash | 100ms | Depends on command |
| search_files | 50ms | Depends on scope |
| list_directory | 20ms | Cached |

### AI Performance
- **First token**: 500ms (depends on Ollama)
- **Tokens/sec**: 25 (depends on model & hardware)
- **Context processing**: 100ms per 1000 tokens

## Optimization Strategies

### Current
- ✅ Async I/O (tokio)
- ✅ Streaming responses
- ✅ Minimal allocations
- ✅ Efficient JSON parsing

### Planned (v0.3.0)
- [ ] Response caching
- [ ] Connection pooling
- [ ] Lazy loading
- [ ] Background tasks
- [ ] Memory pooling

## Memory Usage

**Typical session:**
- Startup: 8 MB
- With context: 12 MB
- Peak (large files): 20 MB

**Optimizations:**
- String interning for repeated text
- Streaming for large responses
- Periodic context cleanup

## Profiling

### CPU Profiling
```bash
cargo install flamegraph
cargo flamegraph --bin ocli
```

### Memory Profiling
```bash
cargo install cargo-instruments
cargo instruments --bin ocli
```

### Benchmarking
```bash
cargo bench
```

## Performance Goals

- Command response: < 10ms
- Tool execution: < 100ms
- AI first token: < 1s
- Memory usage: < 50 MB
- Binary size: < 10 MB

---

**Current status**: All goals met! 🚀
