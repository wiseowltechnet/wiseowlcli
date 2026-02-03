# OCLI Architecture

## Overview

OCLI is a modular AI coding assistant built with Rust, featuring a clean architecture with separation of concerns and well-defined module boundaries.

## System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     User Interface                       │
│  (readline, LCARS styling, TUI, progress indicators)    │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│                  Command Layer                           │
│  (slash commands, suggestions, history, completion)     │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│                   Core Logic                             │
│  (context, planning, tools, streaming, models)          │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│              External Integrations                       │
│  (Ollama API, MCP servers, Git, filesystem)             │
└─────────────────────────────────────────────────────────┘
```

## Module Structure

### Core Modules (src/)

**main.rs** (Entry point)
- CLI argument parsing
- Chat mode orchestration
- Command routing
- Session management

**context.rs** (Context Management)
- Conversation context
- File tracking
- Message history
- Context serialization

**context_manager.rs** (Smart Context)
- Token counting
- Auto-pruning
- Context search
- Summarization

**tools.rs** (Tool System)
- Tool definitions
- Tool execution
- Result handling
- 5 built-in tools

**streaming.rs** (Streaming)
- Response streaming
- Tool call detection
- Real-time output
- Error handling

**planning.rs** (Planning Mode)
- Plan creation
- Step execution
- Progress tracking
- Plan persistence

### UI Modules

**readline.rs** (Input Handling)
- Command history
- Auto-completion
- Hints
- Line editing

**lcars.rs** (Styling)
- LCARS colors
- UI components
- Status indicators
- Themed output

**tui.rs** (Terminal UI)
- Full-screen mode
- Cursor control
- Alternate buffer
- Monitor display

**syntax.rs** (Highlighting)
- Code highlighting
- Language detection
- Theme support
- Terminal colors

**progress.rs** (Progress)
- Spinners
- Progress bars
- ETA display
- Visual feedback

### Feature Modules

**suggestions.rs** (AI Suggestions)
- Typo correction
- Next command prediction
- Workflow recommendations
- Usage learning

**diff_viewer.rs** (Diff Viewing)
- Side-by-side diffs
- Syntax highlighting
- Accept/reject
- LCARS styling

**models.rs** (Multi-Model)
- Model switching
- Model configs
- Temperature control
- Model management

**mcp.rs** (MCP Integration)
- MCP client
- Server discovery
- Tool calling
- Protocol handling

### Support Modules

**wiseowl.rs** (Project Management)
- TODO tracking
- Rules management
- Context notes
- Project state

**stats.rs** (Statistics)
- Session metrics
- Command tracking
- Performance stats
- Usage analytics

**metrics.rs** (Performance)
- Timing metrics
- Resource tracking
- Performance data
- Metric aggregation

**git.rs** (Git Integration)
- Status checking
- Diff viewing
- Commit creation
- Log display

**errors.rs** (Error Handling)
- Error suggestions
- File path hints
- Command corrections
- MCP error formatting

**multi_file.rs** (Multi-File Editing)
- Batch operations
- File coordination
- Change tracking
- Rollback support

**prompts.rs** (AI Prompts)
- System prompts
- Tool descriptions
- MCP integration
- Prompt templates

## Design Patterns

### 1. Command Pattern
Slash commands are discrete, encapsulated actions:
```rust
match command {
    "help" => execute_help(),
    "plan" => execute_plan(),
    // ...
}
```

### 2. Strategy Pattern
Different tool execution strategies:
```rust
trait Tool {
    async fn execute(&self, params: Value) -> Result<Value>;
}
```

### 3. Builder Pattern
Context construction:
```rust
ContextBuilder::new()
    .add_message(msg)
    .add_file(path, content)
    .build()
```

### 4. Observer Pattern
Streaming responses:
```rust
trait StreamObserver {
    fn on_chunk(&mut self, chunk: &str);
    fn on_complete(&mut self);
}
```

### 5. Repository Pattern
Data persistence:
```rust
trait Repository<T> {
    async fn save(&self, item: &T) -> Result<()>;
    async fn load(&self, id: &str) -> Result<T>;
}
```

## Data Flow

### 1. User Input
```
User → readline → Command Parser → Command Handler
```

### 2. AI Interaction
```
User Query → Context Builder → Ollama API → Streaming → Tool Detection → Tool Execution → Response
```

### 3. Tool Execution
```
Tool Call → Tool Router → Tool Implementation → Result → Context Update
```

### 4. MCP Integration
```
MCP Command → MCP Client → Server Discovery → Tool Call → Result
```

## Key Technologies

- **Rust** - Systems programming language
- **Tokio** - Async runtime
- **Reqwest** - HTTP client
- **Serde** - Serialization
- **Crossterm** - Terminal control
- **Rustyline** - Readline functionality
- **Syntect** - Syntax highlighting
- **Indicatif** - Progress indicators

## Performance Characteristics

- **Startup**: < 100ms
- **Command execution**: < 10ms
- **Tool execution**: 10-100ms
- **AI first token**: 500ms (Ollama-dependent)
- **Memory usage**: 8-20 MB
- **Binary size**: 5.6 MB

## Extensibility Points

1. **New Commands** - Add to command match
2. **New Tools** - Implement Tool trait
3. **New MCP Servers** - Add to config
4. **New Models** - Add to model list
5. **New Themes** - Extend LCARS colors

## Security Considerations

- No credential storage
- Sandboxed tool execution
- Input validation
- Safe file operations
- MCP server isolation

## Testing Strategy

- Unit tests per module
- Integration tests for workflows
- Manual testing for UI
- Performance benchmarks
- Security audits

## Future Architecture

### Planned Improvements
- Plugin system
- WebSocket support
- Distributed sessions
- Cloud sync
- Team collaboration

---

**OCLI follows clean architecture principles with clear separation of concerns.** 🏗️
