use std::sync::Arc;

use teloxide::{
    prelude::Requester,
    types::{ChatId, Message},
    Bot,
};

pub struct ResponseSender {
    bot: Arc<Bot>,
}

impl ResponseSender {
    pub fn new(bot: Arc<Bot>) -> Arc<Self> {
        Arc::new(ResponseSender { bot })
    }

    pub async fn send_message(
        &self,
        chat_id: &str,
        text: &str,
    ) -> Result<Message, teloxide::errors::RequestError> {
        self.bot
            .send_message(ChatId(chat_id.parse().unwrap()), text)
            .await
    }
}
