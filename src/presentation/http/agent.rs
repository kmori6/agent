use crate::application::usecase::agent_usecase::AgentUseCase;
use crate::domain::model::message::Message;
use axum::Json;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct RequestAgent {
    model: String,
    input: Vec<Message>,
}

pub async fn agent(Json(payload): Json<RequestAgent>) -> Json<Value> {
    println!("Received request: {:?}", payload);

    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let agent_usecase = AgentUseCase::new(api_key);

    let response_json = agent_usecase
        .run(&payload.model, payload.input)
        .await
        .expect("Failed to run agent usecase");
    println!("Received response: {:?}", response_json);

    Json(response_json)
}
