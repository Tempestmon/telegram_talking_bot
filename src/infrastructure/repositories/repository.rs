use std::error::Error;

use crate::domain::models::Message;

pub trait Repository {
    async fn save_replica(&mut self, replica: Message) -> Result<(), Box<dyn Error>>;
    async fn get_replicas(&self, chat_id: &str) -> Vec<Message>;
    async fn count_replicas(&self, chat_id: &str) -> usize;
    async fn flush_chat(&mut self, chat_id: &str) -> Result<(), Box<dyn Error>>;
}
