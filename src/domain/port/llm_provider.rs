use crate::domain::error::llm_provider_error::LlmProviderError;
use crate::domain::model::agent_item::AgentItem;
use crate::domain::model::function::Function;
use async_trait::async_trait;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn response(
        &self,
        model: &str,
        instruction: &str,
        input: Vec<AgentItem>,
        tools: Vec<Function>,
    ) -> Result<Vec<AgentItem>, LlmProviderError>;
}
