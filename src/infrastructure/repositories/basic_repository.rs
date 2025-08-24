use tracing::info;

use crate::{domain::models::Message, infrastructure::repositories::repository::Repository};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct BasicRepository {
    replicas: Arc<Mutex<HashMap<String, Vec<Message>>>>,
}

impl BasicRepository {
    pub fn new() -> Self {
        info!("Create BasicRepository");
        Self {
            replicas: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Repository for BasicRepository {
    async fn save_replica(&self, replica: Message) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = self.replicas.lock().unwrap();
        let username = &replica.username;
        let chat_id = &replica.chat_id;
        info!("Saving replica from {username} in chat {chat_id}");

        let chat_replicas = storage.entry(replica.chat_id.clone()).or_default();
        chat_replicas.push(replica);
        chat_replicas.sort_by_key(|r| r.time);

        Ok(())
    }

    async fn get_replicas(&self, chat_id: &str) -> Vec<Message> {
        let storage = self.replicas.lock().unwrap();
        storage.get(chat_id).cloned().unwrap_or_default()
    }

    async fn flush_chat(&self, chat_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = self.replicas.lock().unwrap();
        storage.remove(chat_id);
        Ok(())
    }

    async fn count_replicas(&self, chat_id: &str) -> usize {
        let storage = self.replicas.lock().unwrap();
        info!("Storage: {storage:#?}");
        storage.get(chat_id).map(|r| r.len()).unwrap_or(0)
    }
}
