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

// Dashboard benchmarks
fn bench_dashboard_add_response(c: &mut Criterion) {
    c.bench_function("dashboard_add_response", |b| {
        let mut stats = DashboardStats::new();
        b.iter(|| {
            stats.add_response_time(black_box(Duration::from_secs(3)));
        });
    });
}

fn bench_dashboard_avg_calculation(c: &mut Criterion) {
    c.bench_function("dashboard_avg_calculation", |b| {
        let mut stats = DashboardStats::new();
        for i in 0..10 {
            stats.add_response_time(Duration::from_secs(i));
        }
        b.iter(|| {
            black_box(stats.avg_response_time());
        });
    });
}

fn bench_dashboard_add_activity(c: &mut Criterion) {
    c.bench_function("dashboard_add_activity", |b| {
        let mut stats = DashboardStats::new();
        b.iter(|| {
            stats.add_activity(black_box("Test activity".to_string()));
        });
    });
}

criterion_group!(
    dashboard_benches,
    bench_dashboard_add_response,
    bench_dashboard_avg_calculation,
    bench_dashboard_add_activity
);

criterion_main!(dashboard_benches);
