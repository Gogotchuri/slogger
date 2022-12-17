use std::sync::Mutex;

use crate::logger_service::models::Message;

use crate::logger_service::interfaces;

struct InMemoryRepository {
    entries: Mutex<Vec<Message>>,
}
#[must_use]
pub fn new_in_memory() -> impl interfaces::Repository + Send + Sync {
    InMemoryRepository {
        entries: Mutex::new(vec![]),
    }
}

impl interfaces::Repository for InMemoryRepository {
    fn save_message(&self, message: Message) -> Result<(), String> {
        let mut entries = self.entries.lock().unwrap();
        entries.push(message);
        Ok(())
    }
    fn get_messages_for_user(&self, user_id: u32) -> Result<Vec<Message>, String> {
        let entries = self.entries.lock().unwrap();
        let vec = entries
            .iter()
            .filter(|message| message.user_id == user_id)
            .cloned()
            .collect();
        Ok(vec)
    }
}
