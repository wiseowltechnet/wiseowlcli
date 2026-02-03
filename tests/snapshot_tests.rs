use std::process::{Command, Stdio};
use std::io::Write;

#[test]
fn test_version_output() {
    let output = Command::new("./target/release/ocli")
        .arg("--version")
        .output()
        .expect("Failed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    insta::assert_snapshot!(stdout);
}

#[test]
fn test_help_output() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/help\nexit\n").expect("Failed");
    }
    
    let output = child.wait_with_output().expect("Failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Extract just the help section
    if let Some(help_start) = stdout.find("OCLI COMMANDS") {
        let help_section = &stdout[help_start..];
        if let Some(help_end) = help_section.find("You:") {
            insta::assert_snapshot!(&help_section[..help_end]);
        }
    }
}

#[test]
fn test_stats_output() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/stats\nexit\n").expect("Failed");
    }
    
    let output = child.wait_with_output().expect("Failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Extract stats section
    if let Some(stats_start) = stdout.find("Session Statistics") {
        let stats_section = &stdout[stats_start..];
        if let Some(stats_end) = stats_section.find("You:") {
            insta::assert_snapshot!(&stats_section[..stats_end]);
        }
    }
}

#[test]
fn test_config_list_output() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/config list\nexit\n").expect("Failed");
    }
    
    let output = child.wait_with_output().expect("Failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Extract config section
    if let Some(config_start) = stdout.find("Configuration") {
        let config_section = &stdout[config_start..];
        if let Some(config_end) = config_section.find("You:") {
            insta::assert_snapshot!(&config_section[..config_end]);
        }
    }
}

#[test]
fn test_perf_output() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/perf\nexit\n").expect("Failed");
    }
    
    let output = child.wait_with_output().expect("Failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Extract perf section
    if let Some(perf_start) = stdout.find("Performance Metrics") {
        let perf_section = &stdout[perf_start..];
        if let Some(perf_end) = perf_section.find("You:") {
            insta::assert_snapshot!(&perf_section[..perf_end]);
        }
    }
}

#[test]
fn test_models_output() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/models\nexit\n").expect("Failed");
    }
    
    let output = child.wait_with_output().expect("Failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Just verify it runs successfully
    assert!(output.status.success());
}

#[test]
fn test_context_output() {
    let mut child = Command::new("./target/release/ocli")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed");
    
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(b"/context\nexit\n").expect("Failed");
    }
    
    let output = child.wait_with_output().expect("Failed");
    assert!(output.status.success());
}
