use crate::history::{History, HistoryEntry};
use serde_json::json;

pub struct Exporter;

impl Exporter {
    pub fn to_markdown(history: &History) -> String {
        let mut output = String::new();
        output.push_str("# WiseOwl CLI Conversation\n\n");
        
        for (i, entry) in history.iter().enumerate() {
            output.push_str(&format!("## Exchange {}\n\n", i + 1));
            output.push_str(&format!("**User:**\n```\n{}\n```\n\n", entry.command));
            output.push_str(&format!("**Assistant:**\n```\n{}\n```\n\n", entry.response));
            output.push_str("---\n\n");
        }
        
        output
    }
    
    pub fn to_json(history: &History) -> String {
        let entries: Vec<_> = history.iter().map(|e| {
            json!({
                "command": e.command,
                "response": e.response,
                "timestamp": e.timestamp
            })
        }).collect();
        
        json!({
            "version": "0.4.0",
            "entries": entries,
            "count": history.len()
        }).to_string()
    }
    
    pub fn to_text(history: &History) -> String {
        let mut output = String::new();
        output.push_str("WiseOwl CLI Conversation Export\n");
        output.push_str("================================\n\n");
        
        for (i, entry) in history.iter().enumerate() {
            output.push_str(&format!("Exchange {}\n", i + 1));
            output.push_str(&format!("User: {}\n", entry.command));
            output.push_str(&format!("Assistant: {}\n\n", entry.response));
        }
        
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_history() -> History {
        let mut history = History::new(10);
        history.add("test1".to_string(), "response1".to_string());
        history.add("test2".to_string(), "response2".to_string());
        history
    }

    #[test]
    fn test_to_markdown() {
        let history = create_test_history();
        let md = Exporter::to_markdown(&history);
        assert!(md.contains("# WiseOwl CLI Conversation"));
        assert!(md.contains("**User:**"));
        assert!(md.contains("test1"));
    }

    #[test]
    fn test_to_json() {
        let history = create_test_history();
        let json = Exporter::to_json(&history);
        assert!(json.contains("version"));
        assert!(json.contains("test1"));
        assert!(json.contains("response1"));
    }

    #[test]
    fn test_to_text() {
        let history = create_test_history();
        let text = Exporter::to_text(&history);
        assert!(text.contains("WiseOwl CLI Conversation Export"));
        assert!(text.contains("Exchange 1"));
        assert!(text.contains("test1"));
    }

    #[test]
    fn test_empty_history() {
        let history = History::new(10);
        let md = Exporter::to_markdown(&history);
        assert!(md.contains("# WiseOwl CLI Conversation"));
    }
}
