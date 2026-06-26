use axum::Json;
use serde_json::{Value, json};

pub async fn healthcheck() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
