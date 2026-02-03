use std::time::{Duration, Instant};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Metric {
    pub count: u64,
    pub total_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
}

impl Metric {
    fn new() -> Self {
        Self {
            count: 0,
            total_duration: Duration::ZERO,
            min_duration: Duration::MAX,
            max_duration: Duration::ZERO,
        }
    }
    
    fn record(&mut self, duration: Duration) {
        self.count += 1;
        self.total_duration += duration;
        self.min_duration = self.min_duration.min(duration);
        self.max_duration = self.max_duration.max(duration);
    }
    
    pub fn avg_duration(&self) -> Duration {
        if self.count == 0 {
            Duration::ZERO
        } else {
            self.total_duration / self.count as u32
        }
    }
}

pub struct Metrics {
    data: Arc<Mutex<HashMap<String, Metric>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn record(&self, name: &str, duration: Duration) {
        let mut data = self.data.lock().unwrap();
        data.entry(name.to_string())
            .or_insert_with(Metric::new)
            .record(duration);
    }
    
    pub fn get(&self, name: &str) -> Option<Metric> {
        self.data.lock().unwrap().get(name).cloned()
    }
    
    pub fn all(&self) -> HashMap<String, Metric> {
        self.data.lock().unwrap().clone()
    }
    
    pub fn clear(&self) {
        self.data.lock().unwrap().clear();
    }
}

pub struct Timer {
    name: String,
    start: Instant,
    metrics: Arc<Mutex<HashMap<String, Metric>>>,
}

impl Timer {
    pub fn new(name: String, metrics: Arc<Mutex<HashMap<String, Metric>>>) -> Self {
        Self {
            name,
            start: Instant::now(),
            metrics,
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        let mut data = self.metrics.lock().unwrap();
        data.entry(self.name.clone())
            .or_insert_with(Metric::new)
            .record(duration);
    }
}

// Macro for easy timing
#[macro_export]
macro_rules! time {
    ($metrics:expr, $name:expr, $block:block) => {{
        let start = std::time::Instant::now();
        let result = $block;
        $metrics.record($name, start.elapsed());
        result
    }};
}
