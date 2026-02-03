use std::fs;

fn main() {
    let content = fs::read_to_string("src/tools.rs").unwrap();
    println!("Tools module: {} bytes", content.len());
    
    // Test tool call parsing
    let test_call = r#"<tool_call>{"tool":"read_file","parameters":{"path":"test.py"}}</tool_call>"#;
    println!("Test call: {}", test_call);
}
