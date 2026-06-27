use crate::domain::model::input_item::InputItem;
use crate::domain::port::llm_provider::{LlmProvider, LlmProviderFuture};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const OPENAI_RESPONSES_URL: &str = "https://api.openai.com/v1/responses";

#[derive(Debug, Clone)]
pub struct OpenaiLlmProvider {
    api_key: String,
    client: Client,
}

impl OpenaiLlmProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CreateResponseResponse {
    output: Vec<InputItem>,
}

#[derive(Debug, Serialize)]
struct CreateResponseRequest<'a> {
    model: &'a str,
    input: Vec<InputItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    instructions: Option<&'a str>,
}

impl LlmProvider for OpenaiLlmProvider {
    fn response<'a>(
        &'a self,
        model: &'a str,
        instruction: &'a str,
        input: Vec<InputItem>,
    ) -> LlmProviderFuture<'a> {
        Box::pin(async move {
            let body = CreateResponseRequest {
                model,
                input,
                instructions: (!instruction.is_empty()).then_some(instruction),
            };

            let response = self
                .client
                .post(OPENAI_RESPONSES_URL)
                .bearer_auth(&self.api_key)
                .json(&body)
                .send()
                .await?;

            let status = response.status();
            if !status.is_success() {
                let error_body = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "failed to read error response body".to_string());

                return Err(std::io::Error::other(format!(
                    "OpenAI Responses API request failed with {status}: {error_body}",
                ))
                .into());
            }

            let response_body = response.json::<CreateResponseResponse>().await?;

            Ok(response_body.output)
        })
    }
}
