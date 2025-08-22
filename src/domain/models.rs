use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct Message {
    pub username: String,
    pub text: String,
    pub chat_id: String,
    pub time: DateTime<Utc>,
}

impl Message {
    pub fn new(username: &String, text: &String, chat_id: &String, time: DateTime<Utc>) -> Self {
        Self {
            username: username.clone(),
            text: text.clone(),
            chat_id: chat_id.clone(),
            time,
        }
    }
}
