use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::process::Command;

fn bench_startup(c: &mut Criterion) {
    c.bench_function("ocli startup", |b| {
        b.iter(|| {
            Command::new("./target/release/ocli")
                .arg("--version")
                .output()
                .expect("Failed")
        });
    });
}

fn bench_help_command(c: &mut Criterion) {
    c.bench_function("help command", |b| {
        b.iter(|| {
            let mut child = Command::new("./target/release/ocli")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("Failed");
            
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                stdin.write_all(b"/help\nexit\n").expect("Failed");
            }
            
            child.wait_with_output().expect("Failed")
        });
    });
}

fn bench_version_command(c: &mut Criterion) {
    c.bench_function("version command", |b| {
        b.iter(|| {
            let mut child = Command::new("./target/release/ocli")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("Failed");
            
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                stdin.write_all(b"/version\nexit\n").expect("Failed");
            }
            
            child.wait_with_output().expect("Failed")
        });
    });
}

fn bench_stats_command(c: &mut Criterion) {
    c.bench_function("stats command", |b| {
        b.iter(|| {
            let mut child = Command::new("./target/release/ocli")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("Failed");
            
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                stdin.write_all(b"/stats\nexit\n").expect("Failed");
            }
            
            child.wait_with_output().expect("Failed")
        });
    });
}

fn bench_config_operations(c: &mut Criterion) {
    c.bench_function("config set/get", |b| {
        b.iter(|| {
            let mut child = Command::new("./target/release/ocli")
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("Failed");
            
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                stdin.write_all(b"/config set bench_key bench_value\n/config get bench_key\nexit\n")
                    .expect("Failed");
            }
            
            child.wait_with_output().expect("Failed")
        });
    });
}

criterion_group!(
    benches,
    bench_startup,
    bench_help_command,
    bench_version_command,
    bench_stats_command,
    bench_config_operations
);
criterion_main!(benches);
