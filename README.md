# 🦉 OCLI - Ollama Command Line Interface

A Claude Code-like AI coding assistant with self-improvement capabilities, LCARS Star Trek styling, and full terminal UI control.

## ✨ Features

### 🤖 AI-Powered Development
- **Autonomous tool calling** - AI automatically uses tools to read/write files, execute commands
- **Streaming responses** - Real-time output with progress indicators
- **Context-aware** - Maintains conversation history and file context
- **Self-improving** - Can read and modify its own source code

### 🛠️ Built-in Tools
- `read_file` - Read any file with syntax awareness
- `write_file` - Create/modify files with automatic backups
- `execute_bash` - Run shell commands safely
- `search_files` - Find files and content across your project
- `list_directory` - Explore directory structures

### 📋 Planning Mode
- `/plan` - Create AI-generated step-by-step plans
- `/next` - Execute next step in plan
- `/show-plan` - View plan progress

### 🔧 Project Management (WiseOwl)
- Auto-creates `wiseowl/` folder for project tracking
- `/todo` - Add tasks to TODO.md
- `/done` - Mark tasks complete
- `/rule` - Add project rules to RULES.md
- `/context` - Add context to CONTEXT.md

### 🎨 LCARS Interface
- Authentic Star Trek LCARS styling
- RGB colors in multiples of 51 (hex 33)
- Status indicators: ● (blue=success, red=error, purple=info, yellow=warning)
- Clean vector look with high contrast

### 📊 Terminal UI
- `/monitor` - Full-screen real-time statistics (like top/htop)
- Cursor positioning and color control
- Alternate buffer support

### 🔌 MCP (Model Context Protocol) Support
- Load external MCP servers for extended functionality
- `/mcp list` - Show available MCP tools
- `/mcp call <tool> [params]` - Invoke MCP tools
- AI automatically knows about available MCP tools
- Configure servers in `.ocli/mcp_servers.json`

### ⚙️ Configuration & Export
- `/config set <key> <value>` - Set configuration
- `/config get <key>` - Get configuration value
- `/config list` - Show all settings
- `/export [filename]` - Export conversation to markdown

### 📈 Statistics & Git
- `/stats` - Show session statistics
- `/git status|diff|log|commit` - Git integration
- `/version` - Show OCLI version

## 🚀 Installation

### From Source
```bash
git clone https://github.com/yourusername/ollama-ocli.git
cd ollama-ocli
cargo build --release
./target/release/ocli
```

### Homebrew (macOS/Linux)
```bash
brew tap yourusername/ocli
brew install ocli
```

## 📖 Quick Start

1. **Start OCLI**
```bash
ocli
```

2. **Ask the AI to help**
```
You: create a hello world rust program
```

3. **Use planning mode for complex tasks**
```
You: /plan build a web server with authentication
You: /next
```

4. **Monitor your session**
```
You: /monitor
```

5. **Configure MCP servers**
```bash
mkdir -p .ocli
cat > .ocli/mcp_servers.json << JSON
{
  "servers": [
    {
      "name": "filesystem",
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]
    }
  ]
}
JSON
```

## 🎯 Use Cases

### Self-Improvement
OCLI can modify itself:
```
You: add a /version command to show OCLI version
AI: *reads src/main.rs, adds version command, rebuilds*
```

### Project Scaffolding
```
You: /plan create a REST API with user authentication
You: /next
```

### Code Review
```
You: review the code in src/main.rs for potential issues
```

### Debugging
```
You: the tests are failing, can you fix them?
```

## 📝 Slash Commands

| Command | Description |
|---------|-------------|
| `/help` | Show all commands |
| `/plan <task>` | Create step-by-step plan |
| `/next` | Execute next plan step |
| `/show-plan` | View plan progress |
| `/read <file>` | Read file |
| `/write <file>` | Write file |
| `/preview` | Preview pending changes |
| `/apply` | Apply pending changes |
| `/rollback` | Undo last change |
| `/todo <task>` | Add TODO item |
| `/done <id>` | Mark TODO complete |
| `/rule <rule>` | Add project rule |
| `/context <info>` | Add context |
| `/mcp list` | List MCP tools |
| `/mcp call <tool>` | Call MCP tool |
| `/config <cmd>` | Manage settings |
| `/export [file]` | Export conversation |
| `/stats` | Show statistics |
| `/git <cmd>` | Git operations |
| `/monitor` | Real-time monitor |
| `/version` | Show version |
| `/clear` | Clear context |
| `/exit` | Exit OCLI |

## 🎨 LCARS Colors

- **Orange** (#FF9900) - Headers, borders
- **Purple** (#CC99FF) - Session info
- **Blue** (#99CCFF) - Success indicators
- **Yellow** (#FFCC00) - Warnings
- **Red** (#FF6666) - Errors

## 🔧 Configuration

OCLI stores configuration in `.ocli/`:
- `config.json` - User settings
- `mcp_servers.json` - MCP server configuration
- `sessions/` - Conversation history

## 🤝 Contributing

OCLI is designed to be self-improving. You can:
1. Ask OCLI to add features to itself
2. Submit PRs with new capabilities
3. Create MCP servers for extended functionality

## 📜 License

MIT

## 🙏 Acknowledgments

- Built with [Ollama](https://ollama.ai)
- Inspired by Claude Code
- LCARS design from Star Trek
- MCP protocol support

---

**Made with 🦉 by the OCLI community**
