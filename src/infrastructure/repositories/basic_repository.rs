use tracing::info;

use crate::{domain::models::Message, infrastructure::repositories::repository::Repository};
use std::collections::HashMap;

pub struct BasicRepository {
    replicas: HashMap<String, Vec<Message>>,
}

impl BasicRepository {
    pub fn new() -> Self {
        Self {
            replicas: HashMap::new(),
        }
    }
}

impl Repository for BasicRepository {
    async fn save_replica(&mut self, replica: Message) -> Result<(), Box<dyn std::error::Error>> {
        let username = &replica.username;
        let chat_id = &replica.chat_id;
        info!("Saving replica from {username} in chat {chat_id}");

        let chat_replicas = self.replicas.entry(replica.chat_id.clone()).or_default();
        chat_replicas.push(replica);
        chat_replicas.sort_by_key(|r| r.time);

        Ok(())
    }

    async fn get_replicas(&self, chat_id: &str) -> Vec<Message> {
        self.replicas.get(chat_id).cloned().unwrap_or_default()
    }

    async fn flush_chat(&mut self, chat_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.replicas.remove(chat_id);
        Ok(())
    }

    async fn count_replicas(&self, chat_id: &str) -> usize {
        self.replicas.get(chat_id).iter().len()
    }
}
