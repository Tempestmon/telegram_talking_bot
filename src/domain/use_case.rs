use crate::infrastructure::{
    adapters::deepseek::DeepSeekAdapter, repositories::repository::Repository,
};

pub struct ReplyUseCase<R: Repository> {
    deepseek_adapter: DeepSeekAdapter,
    repository: R,
}

impl<R: Repository> ReplyUseCase<R> {
    pub fn new(repository: R) -> Self {
        ReplyUseCase {
            deepseek_adapter: DeepSeekAdapter::new(),
            repository,
        }
    }

    pub async fn execute(&mut self, message: String) -> String {
        _ = self.repository.save_replica(message.clone());
        let previous_replicas = self.repository.get_replicas(3);
        println!("Replicas: {previous_replicas:#?}");
        if previous_replicas.len() >= 3 {
            return self
                .deepseek_adapter
                .get_replica(previous_replicas)
                .await
                .unwrap();
        }
        message
    }
}
