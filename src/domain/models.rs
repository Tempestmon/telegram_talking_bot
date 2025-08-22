use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct Message {
    pub username: String,
    pub text: String,
    pub chat_id: String,
    pub time: DateTime<Utc>,
    pub is_bot_mentioned: bool,
}

impl Message {
    pub fn new(
        username: &String,
        text: &String,
        chat_id: &String,
        time: DateTime<Utc>,
        is_bot_mentioned: bool,
    ) -> Self {
        Self {
            username: username.clone(),
            text: text.clone(),
            chat_id: chat_id.clone(),
            time,
            is_bot_mentioned,
        }
    }
}
