use axum::Json;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
pub struct RequestAgent {
    model: String,
    input: String,
}

pub async fn agent(Json(payload): Json<RequestAgent>) -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
