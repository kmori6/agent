use crate::domain::error::llm_provider_error::LlmProviderError;
use crate::domain::model::input_item::InputItem;
use async_trait::async_trait;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn response(
        &self,
        model: &str,
        instruction: &str,
        input: Vec<InputItem>,
    ) -> Result<Vec<InputItem>, LlmProviderError>;
}
