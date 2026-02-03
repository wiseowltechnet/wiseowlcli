#!/bin/bash
cd ~/projects/ollama-ocli

# Test tool parsing
cat > test_tool_parse.rs << 'RUST'
use std::fs;

fn main() {
    let content = fs::read_to_string("src/tools.rs").unwrap();
    println!("Tools module: {} bytes", content.len());
    
    // Test tool call parsing
    let test_call = r#"<tool_call>{"tool":"read_file","parameters":{"path":"test.py"}}</tool_call>"#;
    println!("Test call: {}", test_call);
}
RUST

rustc test_tool_parse.rs -o test_tool_parse 2>&1 && ./test_tool_parse
