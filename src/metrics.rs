use std::collections::HashMap;

pub struct QualityMetrics {
    syntax_valid: usize,
    syntax_invalid: usize,
    builds_successful: usize,
    builds_failed: usize,
    language_stats: HashMap<String, (usize, usize)>,
}

impl QualityMetrics {
    pub fn new() -> Self {
        Self {
            syntax_valid: 0,
            syntax_invalid: 0,
            builds_successful: 0,
            builds_failed: 0,
            language_stats: HashMap::new(),
        }
    }
    
    pub fn record_syntax(&mut self, language: &str, valid: bool) {
        if valid {
            self.syntax_valid += 1;
        } else {
            self.syntax_invalid += 1;
        }
        
        let stats = self.language_stats.entry(language.to_string()).or_insert((0, 0));
        if valid {
            stats.0 += 1;
        } else {
            stats.1 += 1;
        }
    }
    
    pub fn record_build(&mut self, success: bool) {
        if success {
            self.builds_successful += 1;
        } else {
            self.builds_failed += 1;
        }
    }
    
    pub fn syntax_success_rate(&self) -> f64 {
        let total = self.syntax_valid + self.syntax_invalid;
        if total == 0 {
            0.0
        } else {
            self.syntax_valid as f64 / total as f64
        }
    }
    
    pub fn build_success_rate(&self) -> f64 {
        let total = self.builds_successful + self.builds_failed;
        if total == 0 {
            0.0
        } else {
            self.builds_successful as f64 / total as f64
        }
    }
    
    pub fn language_success_rate(&self, language: &str) -> f64 {
        if let Some((valid, invalid)) = self.language_stats.get(language) {
            let total = valid + invalid;
            if total == 0 {
                0.0
            } else {
                *valid as f64 / total as f64
            }
        } else {
            0.0
        }
    }
    
    pub fn summary(&self) -> String {
        format!(
            "Syntax: {}/{} ({:.1}%) | Builds: {}/{} ({:.1}%)",
            self.syntax_valid,
            self.syntax_valid + self.syntax_invalid,
            self.syntax_success_rate() * 100.0,
            self.builds_successful,
            self.builds_successful + self.builds_failed,
            self.build_success_rate() * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_metrics() {
        let metrics = QualityMetrics::new();
        assert_eq!(metrics.syntax_success_rate(), 0.0);
    }

    #[test]
    fn test_record_syntax() {
        let mut metrics = QualityMetrics::new();
        metrics.record_syntax("rust", true);
        metrics.record_syntax("rust", false);
        assert_eq!(metrics.syntax_success_rate(), 0.5);
    }

    #[test]
    fn test_record_build() {
        let mut metrics = QualityMetrics::new();
        metrics.record_build(true);
        metrics.record_build(true);
        metrics.record_build(false);
        assert_eq!(metrics.build_success_rate(), 2.0 / 3.0);
    }

    #[test]
    fn test_language_stats() {
        let mut metrics = QualityMetrics::new();
        metrics.record_syntax("rust", true);
        metrics.record_syntax("rust", true);
        metrics.record_syntax("python", false);
        assert_eq!(metrics.language_success_rate("rust"), 1.0);
        assert_eq!(metrics.language_success_rate("python"), 0.0);
    }

    #[test]
    fn test_summary() {
        let mut metrics = QualityMetrics::new();
        metrics.record_syntax("rust", true);
        metrics.record_build(true);
        let summary = metrics.summary();
        assert!(summary.contains("100.0%"));
    }
}
