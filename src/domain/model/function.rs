use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}
