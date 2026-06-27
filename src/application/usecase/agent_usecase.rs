use crate::domain::model::input_item::InputItem;
use reqwest::Client;
use serde_json::{Value, json};

pub struct AgentUseCase {
    api_key: String,
    client: Client,
}

impl AgentUseCase {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn run(&self, model: &str, input: Vec<InputItem>) -> Result<Value, reqwest::Error> {
        let body = json!({
            "model": model,
            "input": input
        });

        let response = self
            .client
            .post("https://api.openai.com/v1/responses")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await?;

        let response_json = response.json().await?;
        Ok(response_json)
    }
}
