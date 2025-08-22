use std::error::Error;

use crate::domain::models::Message;

pub trait Repository {
    fn save_replica(&mut self, replica: Message) -> Result<(), Box<dyn Error>>;
    fn get_replicas(&self, count: usize) -> Vec<Message>;
}
