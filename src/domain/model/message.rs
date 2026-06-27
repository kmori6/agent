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
