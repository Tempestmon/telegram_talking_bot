use std::error::Error;

pub trait Repository {
    fn save_replica(&mut self, replica: String) -> Result<(), Box<dyn Error>>;
    fn get_replicas(&self, count: usize) -> Vec<String>;
}
