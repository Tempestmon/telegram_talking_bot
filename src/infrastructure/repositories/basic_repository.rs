use crate::{domain::models::Message, infrastructure::repositories::repository::Repository};
use std::collections::VecDeque;

pub struct BasicRepository {
    replicas: VecDeque<Message>,
}

impl BasicRepository {
    pub fn new() -> Self {
        Self {
            replicas: VecDeque::new(),
        }
    }
}

impl Repository for BasicRepository {
    fn save_replica(&mut self, replica: Message) -> Result<(), Box<dyn std::error::Error>> {
        self.replicas.push_front(replica);
        Ok(())
    }

    fn get_replicas(&self, count: usize) -> Vec<Message> {
        self.replicas.iter().rev().take(count).cloned().collect()
    }
}
