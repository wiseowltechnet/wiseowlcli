# 🦉 WiseOwl CLI

**TDD-First AI Coding Assistant with Performance & Accuracy**

[![Version](https://img.shields.io/badge/version-0.3.2-blue.svg)](https://github.com/wiseowltechnet/wiseowlcli)
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

```bash
git clone https://github.com/wiseowltechnet/wiseowlcli.git
cd wiseowlcli
cargo build --release
./target/release/wiseowlcli
```

## 📊 Performance

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

- [TDD Framework](tests/README.md)
- [Dashboard Features](DASHBOARD_FEATURES.md)
- [Changelog](CHANGELOG.md)

---

**🦉 TDD First • ⚡ Performance Second • 🎯 Accuracy Third**

**Built with 🦉 by WiseOwl Tech**
