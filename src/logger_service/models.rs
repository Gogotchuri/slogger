use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub user_id: u32,
    pub message: String,
}

impl Message {
    pub fn new(user_id: u32, message: String) -> Self {
        Message { user_id, message }
    }
}
