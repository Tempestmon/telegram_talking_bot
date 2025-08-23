use std::error::Error;

use crate::domain::models::Message;

pub trait Repository {
    async fn save_replica(&mut self, replica: Message) -> Result<(), Box<dyn Error>>;
    async fn get_replicas(&self, count: usize) -> Vec<Message>;
    async fn flush_chat(&mut self, chat_id: String) -> Result<(), Box<dyn Error>>;
}
