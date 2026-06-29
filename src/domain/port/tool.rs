use crate::domain::error::tool_error::ToolError;
use crate::domain::model::function::Function;
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;

    fn description(&self) -> &str;

    fn parameters(&self) -> Value;

    fn function(&self) -> Function {
        Function {
            name: self.name().to_string(),
            description: self.description().to_string(),
            parameters: self.parameters(),
        }
    }

    async fn execute(&self, arguments: Value) -> Result<Value, ToolError>;
}
