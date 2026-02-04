use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct HistoryEntry {
    pub command: String,
    pub response: String,
    pub timestamp: u64,
}

pub struct History {
    entries: VecDeque<HistoryEntry>,
    undo_stack: Vec<HistoryEntry>,
    max_size: usize,
}

impl History {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            undo_stack: Vec::new(),
            max_size,
        }
    }
    
    pub fn add(&mut self, command: String, response: String) {
        let entry = HistoryEntry {
            command,
            response,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        self.entries.push_back(entry);
        if self.entries.len() > self.max_size {
            self.entries.pop_front();
        }
        self.undo_stack.clear();
    }
    
    pub fn undo(&mut self) -> Option<HistoryEntry> {
        if let Some(entry) = self.entries.pop_back() {
            self.undo_stack.push(entry.clone());
            Some(entry)
        } else {
            None
        }
    }
    
    pub fn redo(&mut self) -> Option<HistoryEntry> {
        if let Some(entry) = self.undo_stack.pop() {
            self.entries.push_back(entry.clone());
            Some(entry)
        } else {
            None
        }
    }
    
    pub fn get_last(&self) -> Option<&HistoryEntry> {
        self.entries.back()
    }
    
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &HistoryEntry> {
        self.entries.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_history() {
        let history = History::new(10);
        assert_eq!(history.len(), 0);
        assert!(history.is_empty());
    }

    #[test]
    fn test_add_entry() {
        let mut history = History::new(10);
        history.add("test".to_string(), "response".to_string());
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_max_size() {
        let mut history = History::new(2);
        history.add("1".to_string(), "r1".to_string());
        history.add("2".to_string(), "r2".to_string());
        history.add("3".to_string(), "r3".to_string());
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_undo() {
        let mut history = History::new(10);
        history.add("test".to_string(), "response".to_string());
        let entry = history.undo();
        assert!(entry.is_some());
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn test_redo() {
        let mut history = History::new(10);
        history.add("test".to_string(), "response".to_string());
        history.undo();
        let entry = history.redo();
        assert!(entry.is_some());
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_get_last() {
        let mut history = History::new(10);
        history.add("test".to_string(), "response".to_string());
        let last = history.get_last();
        assert!(last.is_some());
        assert_eq!(last.unwrap().command, "test");
    }
}
