use crate::domain::model::input_item::InputItem;
use crate::domain::port::llm_provider::{LlmProvider, LlmProviderError};
use std::sync::Arc;

pub struct AgentUsecase {
    llm_provider: Arc<dyn LlmProvider>,
}

impl AgentUsecase {
    pub fn new(llm_provider: Arc<dyn LlmProvider>) -> Self {
        Self { llm_provider }
    }

    pub async fn run(
        &self,
        model: &str,
        instruction: &str,
        input: Vec<InputItem>,
    ) -> Result<Vec<InputItem>, LlmProviderError> {
        self.llm_provider.response(model, instruction, input).await
    }
}
