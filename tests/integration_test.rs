use std::process::{Command, Stdio};
use std::io::Write;

#[test]
fn test_version_flag() {
    let output = Command::new("./target/release/ocli")
        .arg("--version")
        .output()
        .expect("Failed to execute");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OCLI v0.3.0"));
}

#[test]
fn test_version_short_flag() {
    let output = Command::new("./target/release/ocli")
        .arg("-V")
        .output()
        .expect("Failed to execute");
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OCLI v0.3.0"));
}

#[test]
fn test_help_command() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/help\nexit\n").expect("Failed to write");
    }
    
    let output = child.wait_with_output().expect("Failed to wait");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OCLI COMMANDS"));
}

#[test]
fn test_version_command() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/version\nexit\n").expect("Failed to write");
    }
    
    let output = child.wait_with_output().expect("Failed to wait");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("OCLI v0.3.0"));
}

#[test]
fn test_stats_command() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/stats\nexit\n").expect("Failed to write");
    }
    
    let output = child.wait_with_output().expect("Failed to wait");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Session Statistics"));
}

#[test]
fn test_config_set_get() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/config set test_key test_value\n/config get test_key\nexit\n")
            .expect("Failed to write");
    }
    
    let output = child.wait_with_output().expect("Failed to wait");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("test_key"));
}

#[test]
fn test_alias_commands() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/alias set h /help\n/alias list\nexit\n")
            .expect("Failed to write");
    }
    
    let output = child.wait_with_output().expect("Failed to wait");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Aliases"));
}

#[test]
fn test_history_command() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/history\nexit\n").expect("Failed to write");
    }
    
    let output = child.wait_with_output().expect("Failed to wait");
    assert!(output.status.success());
}

#[test]
fn test_perf_command() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/perf\nexit\n").expect("Failed to write");
    }
    
    let output = child.wait_with_output().expect("Failed to wait");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Performance Metrics"));
}

#[test]
fn test_mcp_list() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/mcp list\nexit\n").expect("Failed to write");
    }
    
    let output = child.wait_with_output().expect("Failed to wait");
    assert!(output.status.success());
}
