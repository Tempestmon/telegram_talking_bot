use env;
use serde_json::{json, Value};
use std::error::Error;

pub struct DeepSeekClient {
    token: String,
    system_prompt: String,
    url: String,
    client: reqwest::Client,
    model: String,
}

impl DeepSeekClient {
    fn new_with_params(
        token: &str,
        system_prompt: &str,
        url: &str,
        client: reqwest::Client,
        model: &str,
    ) -> DeepSeekClient {
        DeepSeekClient {
            token: token.to_string(),
            system_prompt: system_prompt.to_string(),
            url: url.to_string(),
            client,
            model: model.to_string(),
        }
    }

    pub fn new() -> DeepSeekClient {
        Self::new_with_params(
            &env::var("DS_TOKEN").expect("DS_TOKEN is not present"),
            &env::var("DS_SYSTEM_PROMPT").expect("DS_SYSTEM_PROMPT is not present"),
            &env::var("DS_URL").expect("DS_URL is not present"),
            reqwest::Client::new(),
            &env::var("DS_MODEL").expect("DS_MODEL is not present"),
        )
    }

    pub async fn get_completion(self, prompt: &str) -> Result<Value, Box<dyn Error>> {
        let body = json!(
            {
                "messages": [
                    {
                        "role": "system",
                        "content": self.system_prompt
                    },
                    {
                        "role": "user",
                        "content": format!("<telegram messages>{prompt}")
                    },
                ],
                "model": self.model
            }
        );
        let response = self
            .client
            .post(self.url)
            .bearer_auth(self.token)
            .json(&body)
            .send()
            .await?;

        let json_response: Value = response.json().await?;
        Ok(json_response)
    }
}
