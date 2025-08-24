use std::error::Error;

use crate::domain::models::Message;

pub trait Repository: Send + Sync {
    async fn save_replica(&self, replica: Message) -> Result<(), Box<dyn Error>>;
    async fn get_replicas(&self, chat_id: &str) -> Vec<Message>;
    async fn count_replicas(&self, chat_id: &str) -> usize;
    async fn flush_chat(&self, chat_id: &str) -> Result<(), Box<dyn Error>>;
}
