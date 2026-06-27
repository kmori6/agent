use crate::domain::model::input_item::InputItem;
use crate::presentation::state::app_state::AppState;
use axum::{Json, extract::State, http::StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RequestAgent {
    model: String,
    #[serde(default)]
    instructions: String,
    input: Vec<InputItem>,
}

pub async fn agent(
    State(state): State<AppState>,
    Json(payload): Json<RequestAgent>,
) -> Result<Json<Vec<InputItem>>, (StatusCode, String)> {
    println!("Received request: {:?}", payload);

    let RequestAgent {
        model,
        instructions,
        input,
    } = payload;

    let output = state
        .agent_usecase
        .run(&model, &instructions, input)
        .await
        .map_err(|error| (StatusCode::BAD_GATEWAY, error.to_string()))?;

    println!("Received response: {:?}", output);

    Ok(Json(output))
}
