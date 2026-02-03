# Ollama Claude CLI

A simple Claude-like CLI interface for Ollama written in Rust.

## Features
- Interactive chat with any Ollama model
- Streaming responses
- Clean, minimal interface
- Model selection via command line

## Usage

```bash
# Use default model (deepseek-coder:6.7b)
claude

# Use specific model
claude -m llama3.2

# Or
claude --model codellama
```

## Installation

```bash
cargo build --release
cp target/release/ollama-claude ~/.local/bin/claude
```

## Requirements
- Ollama running locally on port 11434
- Rust toolchain for building
