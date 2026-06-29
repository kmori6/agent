use crate::domain::error::llm_provider_error::LlmProviderError;
use crate::domain::model::agent_item::{AgentItem, FunctionCallOutput};
use crate::domain::port::llm_provider::LlmProvider;
use crate::domain::port::tool::Tool;
use log::info;
use std::sync::Arc;

const INSTRUCTION: &str =
    "You are a helpful assistant. Please provide a response based on the following input items.";
const MAX_LOOP: i32 = 20;

pub struct AgentUsecase {
    llm_provider: Arc<dyn LlmProvider>,
    tools: Vec<Arc<dyn Tool>>,
}

impl AgentUsecase {
    pub fn new(llm_provider: Arc<dyn LlmProvider>, tools: Vec<Arc<dyn Tool>>) -> Self {
        Self {
            llm_provider,
            tools,
        }
    }

    pub async fn run(
        &self,
        model: &str,
        input: Vec<AgentItem>,
    ) -> Result<Vec<AgentItem>, LlmProviderError> {
        let tools = self
            .tools
            .iter()
            .map(|tool| tool.function())
            .collect::<Vec<_>>();

        let mut inputs = input.clone();

        for _ in 0..MAX_LOOP {
            info!("LLM input: {:?}", inputs);
            let output = self
                .llm_provider
                .response(model, INSTRUCTION, inputs.clone(), tools.clone())
                .await?;
            info!("LLM output: {:?}", output);

            inputs.extend(output.clone());

            let function_calls = output
                .iter()
                .filter_map(|item| match item {
                    AgentItem::FunctionCall(call) => Some(call.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>();

            if function_calls.is_empty() {
                return Ok(output);
            }

            for function_call in function_calls {
                if let Some(tool) = self
                    .tools
                    .iter()
                    .find(|tool| tool.name() == function_call.name)
                {
                    info!("Function call: {:?}", function_call);
                    let arguments =
                        serde_json::from_str::<serde_json::Value>(&function_call.arguments)
                            .map_err(|e| LlmProviderError::ApiCall(e.to_string()))?;
                    let result = tool
                        .execute(arguments)
                        .await
                        .map_err(|e| LlmProviderError::ApiCall(e.to_string()))?;
                    info!("Function call output: {:?}", result);
                    inputs.push(AgentItem::FunctionCallOutput(FunctionCallOutput {
                        call_id: function_call.call_id.clone(),
                        output: result.to_string(),
                    }));
                }
            }
        }

        Err(LlmProviderError::ApiCall(
            "Exceeded maximum loop count".to_string(),
        ))
    }
}
