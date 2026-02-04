use lru::LruCache;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;

pub struct ResponseCache {
    cache: LruCache<u64, String>,
    hits: usize,
    misses: usize,
}

impl ResponseCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
            hits: 0,
            misses: 0,
        }
    }

    pub fn get(&mut self, prompt: &str, model: &str) -> Option<String> {
        let key = Self::hash_key(prompt, model);
        if let Some(response) = self.cache.get(&key) {
            self.hits += 1;
            Some(response.clone())
        } else {
            self.misses += 1;
            None
        }
    }

    pub fn put(&mut self, prompt: &str, model: &str, response: String) {
        let key = Self::hash_key(prompt, model);
        self.cache.put(key, response);
    }

    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    pub fn stats(&self) -> (usize, usize, f64) {
        (self.hits, self.misses, self.hit_rate())
    }

    fn hash_key(prompt: &str, model: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        prompt.hash(&mut hasher);
        model.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_miss() {
        let mut cache = ResponseCache::new(10);
        assert!(cache.get("test", "model").is_none());
        assert_eq!(cache.misses, 1);
    }

    #[test]
    fn test_cache_hit() {
        let mut cache = ResponseCache::new(10);
        cache.put("test", "model", "response".to_string());
        assert_eq!(cache.get("test", "model"), Some("response".to_string()));
        assert_eq!(cache.hits, 1);
    }

    #[test]
    fn test_hit_rate() {
        let mut cache = ResponseCache::new(10);
        cache.put("test", "model", "response".to_string());
        cache.get("test", "model"); // hit
        cache.get("other", "model"); // miss
        assert_eq!(cache.hit_rate(), 0.5);
    }

    #[test]
    fn test_lru_eviction() {
        let mut cache = ResponseCache::new(2);
        cache.put("1", "m", "r1".to_string());
        cache.put("2", "m", "r2".to_string());
        cache.put("3", "m", "r3".to_string()); // evicts "1"
        assert!(cache.get("1", "m").is_none());
        assert!(cache.get("2", "m").is_some());
    }
}
