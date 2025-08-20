use std::error::Error;

use crate::infrastructure::clients::deepseek::DeepSeekClient;

pub struct DeepSeekAdapter {
    client: DeepSeekClient,
}

impl DeepSeekAdapter {
    pub fn new() -> DeepSeekAdapter {
        DeepSeekAdapter {
            client: DeepSeekClient::new(),
        }
    }

    pub async fn get_replica(self, user_replicas: Vec<String>) -> Result<String, Box<dyn Error>> {
        let prompt = user_replicas.join("\n");
        let prompt = prompt.as_str();
        let raw_response = self.client.get_completion(prompt).await.unwrap();
        let choices = raw_response
            .get("choices")
            .ok_or("Missing choices field")?
            .as_array()
            .ok_or("Choices is not an array")?;
        let first_choice = choices.get(0).ok_or("Choices array is empty")?;
        let content = first_choice
            .get("message")
            .ok_or("Choices array does not have message field")?
            .get("content")
            .ok_or("Message does not have content")?
            .to_string();
        Ok(content)
    }
}
