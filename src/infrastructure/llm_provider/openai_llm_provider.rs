use crate::domain::error::llm_provider_error::LlmProviderError;
use crate::domain::model::agent_item::AgentItem;
use crate::domain::model::function::Function;
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
        input: Vec<AgentItem>,
        tools: Vec<Function>,
    ) -> Result<Vec<AgentItem>, LlmProviderError> {
        let dto_input = input
            .iter()
            .map(|item| match item {
                AgentItem::Message(message) => json!({
                    "type": "message",
                    "role": message.role,
                    "content": message.content,
                }),
                AgentItem::FunctionCall(function_call) => json!({
                    "type": "function_call",
                    "call_id": function_call.call_id,
                    "name": function_call.name,
                    "arguments": function_call.arguments,
                }),
                AgentItem::FunctionCallOutput(function_call_output) => json!({
                    "type": "function_call_output",
                    "call_id": function_call_output.call_id,
                    "output": function_call_output.output,
                }),
            })
            .collect::<Vec<Value>>();

        let dto_tools = tools
            .iter()
            .map(|tool| {
                json!({
                    "type": "function",
                    "name": tool.name,
                    "description": tool.description,
                    "parameters": tool.parameters,
                })
            })
            .collect::<Vec<Value>>();

        let request_body = json!({
            "model": model,
            "input": dto_input,
            "instructions": instruction,
            "tools": dto_tools,
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

        let output = serde_json::from_value::<Vec<AgentItem>>(value_output.clone())
            .map_err(|e| LlmProviderError::ResponseParse(format!("failed to parse output: {e}")))?;

        Ok(output)
    }
}
