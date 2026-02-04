# WiseOwl CLI v0.4.0 Release Notes

## Major Release: Performance, Accuracy & Developer Experience

This release completes Phases 2-4 of the comprehensive improvement plan.

## Highlights

### Performance (Phase 2)
- 50% faster streaming with optimized update intervals
- Parallel tool execution for concurrent MCP operations
- Response caching with LRU eviction
- Automated performance regression tests

### Accuracy (Phase 3)
- Code validation for Rust, Python, JavaScript
- Quality metrics tracking syntax and build success
- Build verification with compile-testing
- Real-time validation during code generation

### Developer Experience (Phase 4)
- Beautiful error messages with context and suggestions
- Undo/redo for command history
- Export conversations to Markdown, JSON, or text

## By The Numbers

- 41 unit tests (up from 8)
- 10 tested modules (up from 1)
- 3 benchmark suites for performance tracking
- 7 feature commits in this release

## Technical Details

New Modules:
- cache.rs - Response caching
- validator.rs - Code validation
- metrics.rs - Quality metrics
- build_verifier.rs - Build verification
- error_formatter.rs - Error formatting
- history.rs - History with undo/redo
- exporter.rs - Conversation export

Performance Benchmarks:
- Cache operations: 6-41ns
- Streaming stats: <10ns
- MCP client creation: <100ns

## Testing

All 41 unit tests passing

## Philosophy

TDD First • Performance Second • Accuracy Third

This release embodies our core philosophy with comprehensive test coverage, measurable performance improvements, and accuracy validation.
