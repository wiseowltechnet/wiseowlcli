use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SessionStats {
    pub messages_sent: usize,
    pub files_read: usize,
    pub files_written: usize,
    pub commands_executed: usize,
}

impl SessionStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display(&self) -> String {
        format!(
            "📊 Session Statistics\n\nMessages: {}\nFiles Read: {}\nFiles Written: {}\nCommands: {}",
            self.messages_sent, self.files_read, self.files_written, self.commands_executed
        )
    }
}
