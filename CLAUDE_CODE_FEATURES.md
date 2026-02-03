# OCLI - Claude Code Features

## Overview
OCLI now implements Claude Code-like functionality with tool calling, multi-file editing, context management, and streaming tool execution.

## Architecture

### Modules

#### 1. tools.rs - Tool Calling System
- Defines available tools: read_file, write_file, execute_bash, search_files, list_directory
- Executes tool calls with structured JSON parameters
- Parses tool calls from AI responses
- Generates system prompts to teach the model about available tools

#### 2. context.rs - Conversation Context Management
- Tracks working files and their contents
- Maintains file change history with timestamps
- Supports rollback to previous file states
- Persists context across sessions

#### 3. streaming.rs - Streaming with Tool Execution
- Streams AI responses in real-time
- Detects tool calls during streaming
- Executes tools immediately when detected

#### 4. multi_file.rs - Multi-File Editor
- Manages pending file edits (create, modify, delete)
- Shows preview of all pending changes
- Applies changes in batch with confirmation

## Usage

### Basic Chat
```bash
ocli                    # Start chat
ocli -m llama3.2        # Use specific model
ocli chat --session work # Named session
```

### Slash Commands
- /help - Show commands
- /read <file> - Load file into context
- /write <file> - Create/modify file with AI
- /preview - Show pending changes
- /apply - Apply pending changes
- /rollback - Undo last change
- /clear - Clear context
- /exit - Exit

## Key Features

1. **Tool Calling**: AI can invoke file operations, bash commands, searches
2. **Multi-File Editing**: Edit multiple files in one conversation
3. **Context Management**: Track files and changes across sessions
4. **Streaming**: Real-time responses with inline tool execution
5. **Safety**: Backups, previews, rollback support

## Testing

Run: ./test_tools.sh

## Build

```bash
cargo build --release
cp target/release/ocli ~/.local/bin/
```
