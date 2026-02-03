# OCLI Feature Roadmap
## Making OCLI like Claude Code CLI + Kiro

### Core Features to Add

#### 1. **File Operations** 
- `ocli read <file>` - Read and analyze files
- `ocli write <file>` - Create/modify files with AI help
- `ocli diff <file>` - Show proposed changes before applying
- File watching for live updates

#### 2. **Project Context**
- `ocli init` - Initialize project context
- Auto-detect project type (Python, Rust, JS, etc.)
- Read project structure and dependencies
- Maintain conversation context across sessions

#### 3. **Code Intelligence**
- Syntax highlighting in terminal
- Code completion suggestions
- Error detection and fixes
- Refactoring suggestions

#### 4. **Interactive Modes**
- `ocli chat` - Current chat mode
- `ocli code` - Code-focused mode with file operations
- `ocli review` - Code review mode
- `ocli debug` - Debug assistance mode

#### 5. **Session Management**
- Save/load conversation history
- Multiple named sessions
- Session templates for different tasks

#### 6. **Integration Features**
- Git integration (commit messages, PR descriptions)
- Terminal command suggestions
- Package manager integration
- Documentation generation

### Implementation Priority

**Phase 1 (Essential)**
- [ ] File read/write operations
- [ ] Project context detection
- [ ] Basic session persistence

**Phase 2 (Enhanced)**
- [ ] Interactive modes
- [ ] Git integration
- [ ] Syntax highlighting

**Phase 3 (Advanced)**
- [ ] Code intelligence features
- [ ] Live file watching
- [ ] Advanced project analysis

### Command Structure
```bash
ocli                    # Interactive chat (current)
ocli read file.py       # Read and analyze file
ocli write file.py      # Create/edit file with AI
ocli init               # Initialize project
ocli --session work     # Named session
ocli --mode code        # Code-focused mode
```

### Technical Requirements
- File system operations
- Project structure analysis
- Session storage (JSON/SQLite)
- Git command integration
- Syntax highlighting library
- Configuration management
