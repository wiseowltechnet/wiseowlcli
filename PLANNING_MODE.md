# Planning Mode

## Overview
OCLI now has Claude Code-style planning mode for complex tasks.

## Commands

### Create a Plan
```bash
# In chat mode
/plan Create a REST API with authentication

# Or standalone
ocli plan Build a CLI tool with subcommands
```

### Execute Plan Steps
```bash
/show-plan    # View current plan
/next         # Execute next step
/next         # Continue...
```

## Example Session

```
$ ocli

You: /plan Create a simple web server in Rust

🎯 Creating plan: Create a simple web server in Rust

📋 Plan: Create a simple web server in Rust

1. ⬜ Create new Cargo project
2. ⬜ Add dependencies (actix-web)
3. ⬜ Create main.rs with basic server
4. ⬜ Add route handlers
5. ⬜ Test the server
6. ⬜ Add error handling

💾 Plan saved. Use /next to execute steps.

You: /next

📍 Step 1: Create new Cargo project

Executing...
🔧 Executing: execute_bash
✅ Result: Created project

✅ Step complete. Use /next for next step.

You: /next

📍 Step 2: Add dependencies (actix-web)
...
```

## Features

- ✅ AI-generated step-by-step plans
- ✅ Automatic tool execution per step
- ✅ Progress tracking with ✅/⬜
- ✅ Plan persistence across sessions
- ✅ Step-by-step execution
- ✅ Completion detection

## Use Cases

1. **Project Creation**
   `/plan Create a CLI tool with config file support`

2. **Feature Addition**
   `/plan Add authentication to existing API`

3. **Refactoring**
   `/plan Refactor monolith into modules`

4. **Learning**
   `/plan Learn Rust async by building examples`

## Plan Storage

Plans saved to: `.ocli/plans/current.json`

## Tips

- Be specific with goals
- Plans work best for 5-10 step tasks
- Use /show-plan to check progress
- Plans persist across sessions
- Each /next executes one step with full tool access
