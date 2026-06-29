use crate::domain::model::agent_item::AgentItem;
use crate::presentation::state::app_state::AppState;
use axum::{Json, extract::State, http::StatusCode};
use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RequestAgent {
    model: String,
    input: Vec<AgentItem>,
}

pub async fn agent(
    State(state): State<AppState>,
    Json(payload): Json<RequestAgent>,
) -> Result<Json<Vec<AgentItem>>, (StatusCode, String)> {
    info!("Received request: {:?}", payload);

    let output = state
        .agent_usecase
        .run(&payload.model, payload.input)
        .await
        .map_err(|error| (StatusCode::BAD_GATEWAY, error.to_string()))?;

    info!("Received response: {:?}", output);

    Ok(Json(output))
}
