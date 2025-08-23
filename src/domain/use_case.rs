use std::sync::Arc;
use tracing::{debug, info};

use crate::{
    domain::models::Message,
    infrastructure::{adapters::deepseek::DeepSeekAdapter, repositories::repository::Repository},
};

pub struct ReplyUseCase<R: Repository> {
    deepseek_adapter: Arc<DeepSeekAdapter>,
    repository: R,
}

impl<R: Repository> ReplyUseCase<R> {
    pub fn new(repository: R) -> Self {
        ReplyUseCase {
            deepseek_adapter: Arc::new(DeepSeekAdapter::new()),
            repository,
        }
    }

    // TODO: Make reply if no messages for a long time
    // TODO: Do not reply for every message. Make debouncing with timer
    pub async fn execute(&mut self, message: Message) -> Option<String> {
        _ = self.repository.save_replica(message.clone()).await;

        let mut previous_replicas = self.repository.get_replicas(20).await;
        let mut ds_text = vec![];
        let chat_id = message.chat_id;
        for replica in &previous_replicas {
            if replica.chat_id == chat_id {
                let username = &replica.username;
                let text = &replica.text;
                ds_text.push(format!("{username}: {text}"));
            }
        }
        previous_replicas.retain(|r| r.chat_id == chat_id);

        let replicas_length = previous_replicas.len();
        info!("Got {replicas_length} replicas for chat {chat_id}");
        debug!("{previous_replicas:#?}");
        if replicas_length >= 3 || message.is_bot_mentioned {
            self.repository
                .flush_chat(chat_id)
                .await
                .expect("Cannot flush repository");
            return Some(
                self.deepseek_adapter
                    .get_replica(ds_text)
                    .await
                    .expect("Error calling deepseek_adapter"),
            );
        }
        None
    }
}
