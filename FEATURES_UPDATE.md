# OCLI v0.2.1 - Features & Homebrew

## ✅ Tasks 4 & 5 Complete!

### Task 4: New Features Added

#### 1. `/history` Command
View past conversation sessions:
```bash
/history
```
Shows last 10 sessions sorted by most recent.

#### 2. `/alias` Command
Create custom shortcuts:
```bash
# Create aliases
/alias set a /apply
/alias set p /preview
/alias set s /stats

# List all aliases
/alias list

# Remove alias
/alias remove a
```

**Use cases:**
- `/alias set q /exit` - Quick exit
- `/alias set h /help` - Quick help
- `/alias set m /monitor` - Quick monitor

### Task 5: Homebrew Tap Created

#### Repository
https://github.com/wiseowltechnet/homebrew-ocli

#### Installation
```bash
brew tap wiseowltechnet/ocli
brew install ocli
```

#### Formula Features
- ✅ Builds from source (v0.2.1)
- ✅ Rust dependency handled
- ✅ Version test included
- ✅ Auto-updates with brew

## 📊 What Changed

| File | Changes |
|------|---------|
| src/main.rs | +82 lines (history, alias commands) |
| Cargo.toml | Version 0.2.0 → 0.2.1 |
| CHANGELOG.md | v0.2.1 entry added |
| README.md | Homebrew instructions updated |

## 🚀 Installation Methods

### 1. Homebrew (macOS/Linux)
```bash
brew tap wiseowltechnet/ocli
brew install ocli
```

### 2. Cargo (from GitHub)
```bash
cargo install --git https://github.com/wiseowltechnet/ollama-ocli
```

### 3. From Source
```bash
git clone https://github.com/wiseowltechnet/ollama-ocli.git
cd ollama-ocli
cargo build --release
./target/release/ocli
```

## 🎯 New Commands Summary

| Command | Description |
|---------|-------------|
| `/history` | View past sessions |
| `/alias set <name> <cmd>` | Create alias |
| `/alias list` | Show aliases |
| `/alias remove <name>` | Remove alias |

## 📈 Version History

- **v0.2.1** (2026-02-03) - History, aliases, Homebrew
- **v0.2.0** (2026-02-03) - MCP, config, export, QA tools
- **v0.1.0** (2026-02-02) - Initial release

## 🎉 Status

- ✅ v0.2.1 released
- ✅ Homebrew tap published
- ✅ New features tested
- ✅ Documentation updated

---

**OCLI now has 27+ commands and Homebrew support!** 🚀
