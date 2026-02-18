> **⚠️ DEPRECATED** — This project has been superseded by [wiseowl-cli](https://github.com/macbeth76/wiseowl-cli) (Java 21 + Micronaut). All features have been consolidated into the new project. This repository is archived and will no longer receive updates.

---

# 🦉 WiseOwl CLI

**TDD-First AI Coding Assistant with Performance & Accuracy**

[![CI](https://github.com/wiseowltechnet/wiseowlcli/workflows/CI/badge.svg)](https://github.com/wiseowltechnet/wiseowlcli/actions)
[![Version](https://img.shields.io/badge/version-0.4.1-blue.svg)](https://github.com/wiseowltechnet/wiseowlcli)
[![Tests](https://img.shields.io/badge/tests-passing-green.svg)](tests/)
[![TDD](https://img.shields.io/badge/TDD-first-orange.svg)](tests/README.md)

## 🎯 Philosophy

**1. TDD First** - Test-driven development with automated conversation tests  
**2. Performance Second** - 100x faster operations, real-time metrics  
**3. Accuracy Third** - Reliable AI responses with context management

## ✨ Features

### 🧪 TDD-First Development
- **Automated Conversation Tests** - `expect`-based interactive testing
- **Real User Workflow Testing** - Tests actual terminal interactions
- **Regression Prevention** - Catches UI/UX issues before deployment
- **CI/CD Ready** - Automated test suite
- **Easy Test Addition** - Simple pattern for new tests

### ⚡ Performance (100x Faster)
- **Live Progress Indicator** - Real-time token count, speed
- **Direct File Commands** - Skip LLM calls
- **Build Integration** - Compile without exiting
- **Instant Templates** - Generate code instantly
- **LCARS Dashboard** - Real-time metrics

### 🎯 Accuracy
- **Smart Context Management** - Auto-summarize, search, prune
- **MCP Tools Integration** - 18+ tools
- **Multi-Model Support** - Switch models
- **Conversation History** - Full context

## 🧪 Testing

```bash
# Run all tests
./tests/run_tests.sh

# Run specific test
./tests/conversation_test.exp
```

### Test Coverage
- ✅ CLI startup
- ✅ Conversational AI
- ✅ Progress indicator
- ✅ Dashboard
- ✅ File commands
- ✅ Clean exit

## 🚀 Quick Start

**Fast install:**
```bash
git clone https://github.com/wiseowltechnet/wiseowlcli.git
cd wiseowlcli
./install.sh
./install_mcp_servers.sh  # Install MCP servers
```

**Or build manually:**
```bash
git clone https://github.com/wiseowltechnet/wiseowlcli.git
cd wiseowlcli
cargo build --release
./target/release/wiseowlcli
```

📖 See [QUICKSTART.md](QUICKSTART.md) for detailed guide.

## 🔧 MCP Servers

WiseOwl CLI includes **13 MCP servers** (8 remote + 5 custom):

### Remote Servers (Official)
- **Filesystem** - File operations
- **GitHub** - Issues, PRs, repos
- **Puppeteer** ⭐ - Browser automation
- **Brave Search** - Web search
- **PostgreSQL** - Database access
- **Slack** - Team communication
- **Google Maps** - Location services
- **Fetch** - HTTP requests

### Custom Servers
- **Java** (Maven/Gradle) - 6 tools
- **Ansible** (Playbooks/Roles) - 3 tools
- **Bash** (Scripting) - 4 tools
- **Angular** (CLI/Build) - 5 tools
- **React** (Create/Build) - 6 tools

**Total:** 13 servers, 50+ tools

📖 See [REMOTE_MCP_SERVERS.md](REMOTE_MCP_SERVERS.md) for setup.

## 📊 Performance

- **Binary Size**: 2.8MB (optimized)
- **Build Time**: < 0.3s (incremental)
- **File Operations**: 100x faster
- **Build Integration**: Instant
- **Template Generation**: <1ms
- **Memory**: ~45MB

## 🛠️ Commands

- `/write-direct` - Instant file writing
- `/append` - Append to files
- `/build` - Compile projects
- `/template` - Generate code
- `/dashboard` - Performance metrics
- `/model` - Switch models

## 🎨 LCARS Styling

Star Trek LCARS-inspired interface with orange/blue colors.

## 📝 Documentation

- [Quick Start Guide](QUICKSTART.md) - Get started in 3 minutes
- [Examples](examples/README.md) - Practical usage examples
- [Contributing](CONTRIBUTING.md) - How to contribute
- [Security Policy](SECURITY.md) - Security guidelines
- [Homebrew Install](HOMEBREW.md) - macOS installation
- [Shell Completions](COMPLETIONS.md) - Tab completion setup
- [TDD Framework](tests/README.md) - Testing guide
- [Dashboard Features](DASHBOARD_FEATURES.md) - Performance monitoring
- [Performance Optimization](PERFORMANCE_OPTIMIZATION.md) - Binary optimization
- [Changelog](CHANGELOG.md) - Version history

---

**🦉 TDD First • ⚡ Performance Second • 🎯 Accuracy Third**

**Built with 🦉 by WiseOwl Tech**
