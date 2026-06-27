use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageContent {
    InputText { text: String },
    OutputText { text: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    role: Role,
    content: Vec<MessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub call_id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallOutput {
    pub call_id: String,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputItem {
    Message(Message),
    FunctionCall(FunctionCall),
    FunctionCallOutput(FunctionCallOutput),
}
