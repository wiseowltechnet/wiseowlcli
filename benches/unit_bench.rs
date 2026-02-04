use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

// Mock DashboardStats for benchmarking
struct DashboardStats {
    response_times: Vec<f64>,
    activity_log: Vec<String>,
}

impl DashboardStats {
    fn new() -> Self {
        Self {
            response_times: Vec::new(),
            activity_log: Vec::new(),
        }
    }

    fn add_response_time(&mut self, duration: Duration) {
        self.response_times.push(duration.as_secs_f64());
        if self.response_times.len() > 10 {
            self.response_times.remove(0);
        }
    }

    fn avg_response_time(&self) -> f64 {
        if self.response_times.is_empty() {
            return 0.0;
        }
        self.response_times.iter().sum::<f64>() / self.response_times.len() as f64
    }

    fn add_activity(&mut self, msg: String) {
        self.activity_log.push(msg);
        if self.activity_log.len() > 10 {
            self.activity_log.remove(0);
        }
    }
}

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
    benches,
    bench_dashboard_add_response,
    bench_dashboard_avg_calculation,
    bench_dashboard_add_activity
);

criterion_group!(benches,
    streaming_benches::bench_stream_stats_new, dashboard_benchmarks::bench_dashboard_add_response, dashboard_benchmarks::bench_dashboard_avg_calculation, dashboard_benchmarks::bench_dashboard_add_activity, mcp_benches::bench_mcp_client_new);
criterion_main!(benches);

mod mcp_benches {
    use criterion::{black_box, Criterion};
    use wiseowlcli::mcp::MCPClient;

    pub fn bench_mcp_client_new(c: &mut Criterion) {
        c.bench_function("mcp_client_new", |b| {
            b.iter(|| {
                black_box(MCPClient::new())
            })
        });
    }
}

pub fn mcp_benchmarks(c: &mut Criterion) {
    mcp_benches::bench_mcp_client_new(c);
}

mod streaming_benches {
    use criterion::{black_box, Criterion};
    use wiseowlcli::streaming::StreamStats;

    pub fn bench_stream_stats_new(c: &mut Criterion) {
        c.bench_function("stream_stats_new", |b| {
            b.iter(|| {
                black_box(StreamStats::new(100, 10.0))
            })
        });
    }
}

pub fn streaming_benchmarks(c: &mut Criterion) {
    streaming_benches::bench_stream_stats_new(c);
}
