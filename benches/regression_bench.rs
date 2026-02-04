use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use wiseowlcli::{cache::ResponseCache, mcp::MCPClient, streaming::StreamStats};

fn cache_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_regression");
    
    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut cache = ResponseCache::new(size);
                for i in 0..size {
                    cache.put(&format!("prompt{}", i), "model", format!("response{}", i));
                }
                cache.hit_rate()
            });
        });
    }
    group.finish();
}

fn mcp_benchmarks(c: &mut Criterion) {
    c.bench_function("mcp_client_creation", |b| {
        b.iter(|| MCPClient::new())
    });
}

fn streaming_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("streaming_regression");
    
    for tokens in [100, 1000, 10000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(tokens), tokens, |b, &tokens| {
            b.iter(|| StreamStats::new(tokens, 10.0));
        });
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .significance_level(0.1)
        .sample_size(100);
    targets = cache_benchmarks, mcp_benchmarks, streaming_benchmarks
);
criterion_main!(benches);
