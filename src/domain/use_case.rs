use crate::infrastructure::adapters::deepseek::DeepSeekAdapter;

pub struct ReplyUseCase {
    deepseek_adapter: DeepSeekAdapter,
}

impl ReplyUseCase {
    pub fn new() -> ReplyUseCase {
        ReplyUseCase {
            deepseek_adapter: DeepSeekAdapter::new(),
        }
    }

    pub async fn execute(&self, message: String) -> String {
        message
    }
}
