use crate::infrastructure::repositories::repository::Repository;
use std::collections::VecDeque;

pub struct BasicRepository {
    replicas: VecDeque<String>,
}

impl BasicRepository {
    pub fn new() -> Self {
        Self {
            replicas: VecDeque::new(),
        }
    }
}

impl Repository for BasicRepository {
    fn save_replica(&mut self, replica: String) -> Result<(), Box<dyn std::error::Error>> {
        self.replicas.push_front(replica);
        Ok(())
    }

    fn get_replicas(&self, count: usize) -> Vec<String> {
        self.replicas.iter().rev().take(count).cloned().collect()
    }
}
