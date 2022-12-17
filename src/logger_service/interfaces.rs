use crate::logger_service::models::Message;

pub trait Service {
    fn log(&self, message: Message) -> Result<(), String>;
    fn get_messages_for_user(&self, user_id: u32) -> Result<Vec<Message>, String>;
    fn ping(&self) -> bool;
}
pub trait Repository {
    fn save_message(&self, message: Message) -> Result<(), String>;
    fn get_messages_for_user(&self, user_id: u32) -> Result<Vec<Message>, String>;
}

pub trait Port {
    fn log(&self, message: Message);
    fn ping(&self) -> bool;
}
