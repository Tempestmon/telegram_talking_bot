use std::sync::Arc;
use tracing::info;

use crate::{
    domain::models::Message,
    infrastructure::{adapters::deepseek::DeepSeekAdapter, repositories::repository::Repository},
};

pub struct ReplyUseCase<R: Repository> {
    deepseek_adapter: Arc<DeepSeekAdapter>,
    repository: Arc<R>,
}

impl<R: Repository> ReplyUseCase<R> {
    pub fn new(deepseek_adapter: Arc<DeepSeekAdapter>, repository: Arc<R>) -> Self {
        ReplyUseCase {
            deepseek_adapter,
            repository,
        }
    }

    // TODO: Make reply if no messages for a long time
    // TODO: Do not reply for every message. Make debouncing with timer
    pub async fn execute(&self, message: Message) -> Option<String> {
        _ = self.repository.save_replica(message.clone()).await;

        let chat_id = message.chat_id.as_str();
        let previous_replicas = self.repository.get_replicas(chat_id).await;
        let mut ds_text = vec![];
        for replica in &previous_replicas {
            if replica.chat_id == chat_id {
                let username = &replica.username;
                let text = &replica.text;
                ds_text.push(format!("{username}: {text}"));
            }
        }

        let replicas_length = self.repository.count_replicas(chat_id).await;
        info!("Got {replicas_length} replicas for chat {chat_id}");
        if replicas_length >= 3 || message.is_bot_mentioned || message.is_private {
            self.repository
                .flush_chat(chat_id)
                .await
                .expect("Cannot flush repository");
            let bot_replica = self
                .deepseek_adapter
                .get_replica(ds_text)
                .await
                .expect("Error calling deepseek_adapter");
            // TODO: Save bot replica
            return Some(bot_replica);
        }
        None
    }
}
