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
        username: &str,
        text: &str,
        chat_id: &str,
        time: DateTime<Utc>,
        is_bot_mentioned: bool,
    ) -> Self {
        Self {
            username: username.to_owned(),
            text: text.to_owned(),
            chat_id: chat_id.to_owned(),
            time,
            is_bot_mentioned,
        }
    }
}
