use crate::infrastructure::adapters::deepseek::DeepSeekAdapter;

struct ReplyUseCase {
    deepseek_adapter: DeepSeekAdapter,
}

impl ReplyUseCase {
    pub async fn execute(self, message: String) -> String {}
}

#[warn(dead_code)]
struct StartConversationUseCase {}
