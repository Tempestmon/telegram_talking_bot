use crate::{
    domain::models::Message,
    infrastructure::{adapters::deepseek::DeepSeekAdapter, repositories::repository::Repository},
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

    // TODO: Insta-reply if mentioned
    // TODO: Make reply if no messages for a long time
    // TODO: Do not reply for every message. Make debouncing
    pub async fn execute(&mut self, message: Message) -> Option<String> {
        _ = self.repository.save_replica(message.clone());
        let previous_replicas = self.repository.get_replicas(20);
        let mut ds_text = vec![];
        for replica in &previous_replicas {
            let username = &replica.username;
            let text = &replica.text;
            ds_text.push(format!("{username}: {text}"));
        }
        if previous_replicas.len() >= 3 {
            return Some(
                self.deepseek_adapter
                    .get_replica(ds_text)
                    .await
                    .expect("Error calling deepseek_adapter"),
            );
        }
        None
    }
}
