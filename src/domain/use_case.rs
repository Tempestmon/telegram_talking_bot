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
    // TODO: Do not reply for every message. Make debouncing
    pub async fn execute(&mut self, message: Message) -> Option<String> {
        _ = self.repository.save_replica(message.clone()).await;

        let previous_replicas = self.repository.get_replicas(20).await;
        let mut ds_text = vec![];
        for replica in &previous_replicas {
            if replica.chat_id == message.chat_id {
                let username = &replica.username;
                let text = &replica.text;
                ds_text.push(format!("{username}: {text}"));
            }
        }

        let replicas_length = previous_replicas.len();
        info!("Got {replicas_length} replicas");
        debug!("{previous_replicas:#?}");
        if replicas_length >= 5 || message.is_bot_mentioned {
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
