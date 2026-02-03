# Contributing to OCLI

Thanks for your interest in contributing to OCLI! 🦉

## Ways to Contribute

1. **Use OCLI to improve itself** - The most meta way! Ask OCLI to add features
2. **Report bugs** - Open an issue with details
3. **Submit PRs** - Fix bugs or add features
4. **Create MCP servers** - Extend OCLI with new tools
5. **Improve documentation** - Help others understand OCLI

## Development Setup

```bash
git clone https://github.com/wiseowltechnet/ollama-ocli.git
cd ollama-ocli
cargo build
./target/debug/ocli
```

## Self-Improvement Example

```
You: add a /history command to show past conversations
AI: *reads src/main.rs, adds history command, tests it*
```

## Code Style

- Minimal implementations
- Clear error messages
- LCARS styling for UI
- Progress indicators for long operations

## Testing

```bash
cargo test
cargo build --release
./target/release/ocli
```

## Pull Request Process

1. Fork the repo
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit PR with clear description

## Questions?

Open an issue or ask OCLI itself! 😄
