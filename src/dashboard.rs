use std::time::{Duration, Instant};
use sysinfo::System;

pub struct DashboardStats {
    pub response_times: Vec<f64>,
    pub memory_usage: u64,
    pub token_count: usize,
    pub turn_count: usize,
    pub start_time: Instant,
    pub activity_log: Vec<String>,
}

impl DashboardStats {
    pub fn new() -> Self {
        Self {
            response_times: Vec::new(),
            memory_usage: 0,
            token_count: 0,
            turn_count: 0,
            start_time: Instant::now(),
            activity_log: Vec::new(),
        }
    }

    pub fn add_response_time(&mut self, duration: Duration) {
        self.response_times.push(duration.as_secs_f64());
        if self.response_times.len() > 10 {
            self.response_times.remove(0);
        }
    }

    pub fn add_activity(&mut self, msg: String) {
        self.activity_log.push(format!("[{}] {}", 
            chrono::Local::now().format("%H:%M:%S"), msg));
        if self.activity_log.len() > 10 {
            self.activity_log.remove(0);
        }
    }

    pub fn update_memory(&mut self) {
        let mut sys = System::new_all();
        sys.refresh_all();
        if let Some(process) = sys.process(sysinfo::Pid::from_u32(std::process::id())) {
            self.memory_usage = process.memory();
        }
    }

    pub fn avg_response_time(&self) -> f64 {
        if self.response_times.is_empty() {
            return 0.0;
        }
        self.response_times.iter().sum::<f64>() / self.response_times.len() as f64
    }

    pub fn uptime(&self) -> Duration {
        self.start_time.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_new_dashboard_stats() {
        let stats = DashboardStats::new();
        assert_eq!(stats.response_times.len(), 0);
        assert_eq!(stats.token_count, 0);
        assert_eq!(stats.turn_count, 0);
    }

    #[test]
    fn test_add_response_time() {
        let mut stats = DashboardStats::new();
        stats.add_response_time(Duration::from_secs(3));
        assert_eq!(stats.response_times.len(), 1);
        assert_eq!(stats.response_times[0], 3.0);
    }

    #[test]
    fn test_response_time_limit() {
        let mut stats = DashboardStats::new();
        for i in 0..15 {
            stats.add_response_time(Duration::from_secs(i));
        }
        assert_eq!(stats.response_times.len(), 10);
        assert_eq!(stats.response_times[0], 5.0); // First 5 removed
    }

    #[test]
    fn test_avg_response_time() {
        let mut stats = DashboardStats::new();
        stats.add_response_time(Duration::from_secs(2));
        stats.add_response_time(Duration::from_secs(4));
        assert_eq!(stats.avg_response_time(), 3.0);
    }

    #[test]
    fn test_avg_response_time_empty() {
        let stats = DashboardStats::new();
        assert_eq!(stats.avg_response_time(), 0.0);
    }

    #[test]
    fn test_add_activity() {
        let mut stats = DashboardStats::new();
        stats.add_activity("Test activity".to_string());
        assert_eq!(stats.activity_log.len(), 1);
        assert!(stats.activity_log[0].contains("Test activity"));
    }

    #[test]
    fn test_activity_log_limit() {
        let mut stats = DashboardStats::new();
        for i in 0..15 {
            stats.add_activity(format!("Activity {}", i));
        }
        assert_eq!(stats.activity_log.len(), 10);
    }

    #[test]
    fn test_uptime() {
        let stats = DashboardStats::new();
        std::thread::sleep(Duration::from_millis(100));
        assert!(stats.uptime().as_millis() >= 100);
    }
}
