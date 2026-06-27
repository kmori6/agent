use crate::domain::error::llm_provider_error::LlmProviderError;
use crate::domain::model::input_item::InputItem;
use crate::domain::port::llm_provider::LlmProvider;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{Value, json};

const OPENAI_RESPONSES_URL: &str = "https://api.openai.com/v1/responses";

#[derive(Debug, Clone)]
pub struct OpenaiLlmProvider {
    api_key: String,
    client: Client,
}

impl OpenaiLlmProvider {
    pub fn new(api_key: String, client: Client) -> Self {
        Self { api_key, client }
    }
}

#[async_trait]
impl LlmProvider for OpenaiLlmProvider {
    async fn response(
        &self,
        model: &str,
        instruction: &str,
        input: Vec<InputItem>,
    ) -> Result<Vec<InputItem>, LlmProviderError> {
        let request_body = json!({
            "model": model,
            "input": input,
            "instructions": instruction,
        });

        let response = self
            .client
            .post(OPENAI_RESPONSES_URL)
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| LlmProviderError::ApiCall(format!("failed to send request: {e}")))?;

        if !response.status().is_success() {
            return Err(LlmProviderError::ApiCall(format!(
                "failed to call LLM API: status code {}",
                response.status()
            )));
        }

        let response_body = response.json::<Value>().await.map_err(|e| {
            LlmProviderError::ResponseParse(format!("failed to parse response: {e}"))
        })?;

        let value_output = response_body
            .get("output")
            .ok_or_else(|| LlmProviderError::ResponseParse("missing output field".to_string()))?;

        let output = serde_json::from_value::<Vec<InputItem>>(value_output.clone())
            .map_err(|e| LlmProviderError::ResponseParse(format!("failed to parse output: {e}")))?;

        Ok(output)
    }
}
