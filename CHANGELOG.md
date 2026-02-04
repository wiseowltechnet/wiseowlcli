# Changelog

## [0.4.0] - 2026-02-04

### Added - Phase 2: Performance
- **Response Caching**: LRU cache with O(1) lookups, hit/miss tracking
- **Parallel Tool Execution**: Concurrent MCP tool calls with 30s timeout
- **Streaming Optimization**: Reduced update frequency (20 tokens), 512-byte buffering
- **Performance Regression Tests**: Automated benchmarks for cache, MCP, streaming

### Added - Phase 3: Accuracy
- **Code Validator**: Heuristic-based validation for Rust, Python, JavaScript
- **Quality Metrics**: Track syntax validation and build success rates per language
- **Validated Streaming**: Auto-validate generated code during streaming
- **Build Verifier**: Compile-test generated code with rustc/py_compile

### Added - Phase 4: Developer Experience
- **Error Formatter**: Formatted error messages with context and suggestions
- **History with Undo/Redo**: Command history with undo/redo functionality
- **Conversation Exporter**: Export conversations to Markdown, JSON, or plain text

### Testing
- 41 unit tests passing
- 3 benchmark suites (unit, regression)
- 10 modules with comprehensive test coverage

### Modules
- `cache.rs`: Response caching (4 tests)
- `mcp.rs`: Parallel tool execution (2 tests)
- `streaming.rs`: Optimized streaming with validation (7 tests)
- `validator.rs`: Code validation (6 tests)
- `metrics.rs`: Quality metrics tracking (5 tests)
- `build_verifier.rs`: Build verification (3 tests)
- `error_formatter.rs`: Error formatting (4 tests)
- `history.rs`: History with undo/redo (6 tests)
- `exporter.rs`: Conversation export (4 tests)

### Performance Improvements
- 2x faster streaming updates (10→20 token interval)
- Buffered I/O reduces system calls
- Parallel MCP tool execution
- Response caching for repeated queries

## [0.3.2] - Previous
- LCARS dashboard with live progress
- Direct file commands
- Template generation

