use crate::domain::error::tool_error::ToolError;
use async_trait::async_trait;
use serde_json::{Value, json};

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;

    fn description(&self) -> &str;

    fn parameters(&self) -> Value;

    fn spec(&self) -> Value {
        json!({
            "name": self.name(),
            "description": self.description(),
            "parameters": self.parameters(),
        })
    }

    async fn execute(&self, arguments: Value) -> Result<Value, ToolError>;
}
