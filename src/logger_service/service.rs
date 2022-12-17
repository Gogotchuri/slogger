use std::sync::Arc;

// Clean architecture service layer
use crate::logger_service::interfaces;

use crate::logger_service::models::Message;

#[derive(Clone)]
pub struct Service {
    repository: Arc<dyn interfaces::Repository + Send + Sync>,
}

#[must_use]
pub fn new(
    repository: impl interfaces::Repository + Send + Sync + 'static,
) -> impl interfaces::Service + Send + Sync + 'static {
    Service {
        repository: Arc::new(repository),
    }
}

impl interfaces::Service for Service {
    fn log(&self, message: Message) -> Result<(), String> {
        self.repository.save_message(message)
    }
    fn ping(&self) -> bool {
        true
    }

    fn get_messages_for_user(&self, user_id: u32) -> Result<Vec<Message>, String> {
        self.repository.get_messages_for_user(user_id)
    }
}
