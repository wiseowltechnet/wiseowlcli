# 🦉 WiseOwl CLI

**Enterprise-grade AI coding assistant with LCARS styling, real-time dashboard, and MCP tools integration**

[![Version](https://img.shields.io/badge/version-0.3.2-blue.svg)](https://github.com/wiseowltechnet/wiseowlcli)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

## ✨ Features

### 🎨 LCARS Dashboard (NEW in v0.3.2)
- **Live Progress Indicator** - Real-time token count, speed, and elapsed time
- **Full-Screen Dashboard** - `/dashboard` command with LCARS styling
- **Performance Metrics** - Response time graphs, memory usage, activity log
- **System Stats** - MCP tools status, session statistics

### 🚀 Core Features
- **Conversational AI** - Natural language coding assistant
- **MCP Tools Integration** - 18+ tools including filesystem, git, tech books
- **Direct File Commands** - `/write-direct`, `/append`, `/build`, `/template`
- **Multi-Model Support** - Switch between Ollama models with `/model`
- **Command History** - Navigate with ↑/↓ arrows
- **Syntax Highlighting** - Colored code blocks
- **Smart Context** - Auto-summarize, search, prune

## 🎯 Quick Start

### Installation
```bash
git clone https://github.com/wiseowltechnet/wiseowlcli.git
cd wiseowlcli
cargo build --release
./target/release/wiseowlcli
```

### Usage
```bash
# Start WiseOwl CLI
wiseowlcli

# Ask questions
You: How do I create a REST API in Go?

# Watch live progress
╔══════════════════════════════════════════════════════════╗
║ ⏳ 45 tokens | 15.2 tok/s | 3.1s elapsed                ║
╚══════════════════════════════════════════════════════════╝

# Open dashboard
You: /dashboard
[Press Esc to exit]

# Use direct commands
You: /template go-mcp-server server.go
You: /build go
You: /write-direct README.md "# My Project"
```

## 📊 Dashboard Features

### Live Progress
Shows real-time stats while AI generates responses:
- Token count (updates every 10 tokens)
- Tokens per second
- Elapsed time

### Full Dashboard (`/dashboard`)
- Response time graphs (Unicode bars)
- Memory usage gauge
- Activity log (last 10 events)
- MCP tools status
- Session statistics

## 🛠️ Commands

### Conversational
- Natural language questions
- Multi-turn conversations
- Context-aware responses

### Direct File Operations
- `/write-direct <file> <content>` - Instant file writing (100x faster)
- `/append <file> <content>` - Append to files
- `/build [go|rust|npm|auto]` - Compile projects
- `/template <type> <path>` - Generate from templates

### System
- `/dashboard` - Open full-screen dashboard
- `/model <name>` - Switch models
- `/help` - Show all commands
- `/exit` - Quit

## 🎨 LCARS Styling

WiseOwl CLI features Star Trek LCARS-inspired interface:
- Orange (255,153,0) and blue (153,204,255) color scheme
- Unicode box drawing characters
- Real-time updating displays
- Clean, futuristic aesthetic

## 🔧 MCP Tools

Integrated Model Context Protocol tools:
- **Tech Books** - Search and extract from technical books
- **Filesystem** - Read, write, search files
- **Git** - Repository operations
- **And more** - 18+ tools available

## 📈 Performance

- **Live Progress**: Updates every 10 tokens (~0.1ms overhead)
- **Memory**: ~45MB typical usage
- **Response Time**: Tracked and displayed in dashboard
- **Build Time**: ~12s for release build

## 🚀 Recent Updates

### v0.3.2 (2026-02-03)
- ✨ Live progress indicator during AI responses
- ✨ Full LCARS dashboard with `/dashboard` command
- ✨ Real-time stats tracking
- ✨ Response time graphs
- ✨ Memory usage monitoring

### v0.3.1 (2026-02-03)
- ⚡ Direct file commands (`/write-direct`, `/append`)
- ⚡ Build integration (`/build`)
- ⚡ Code templates (`/template`)
- ⚡ Model switching (`/model`)

### v0.3.0 (2026-02-03)
- 🎉 Command history with ↑/↓
- 🎉 Auto-completion
- 🎉 Syntax highlighting
- 🎉 Smart context management

## 📝 Documentation

- [Dashboard Features](DASHBOARD_FEATURES.md)
- [Conversation Test](CONVERSATION_TEST.md)
- [Changelog](CHANGELOG.md)

## 🤝 Contributing

Contributions welcome! This is a fork of ollama-ocli with enhanced features.

## 📄 License

MIT License - See [LICENSE](LICENSE) file

## 🔗 Links

- **GitHub**: https://github.com/wiseowltechnet/wiseowlcli
- **Original**: https://github.com/wiseowltechnet/ollama-ocli

---

**Built with 🦉 by WiseOwl Tech**
