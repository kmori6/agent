use crate::application::usecase::agent_usecase::AgentUsecase;
use crate::domain::port::llm_provider::LlmProvider;
use crate::infrastructure::llm_provider::openai_llm_provider::OpenaiLlmProvider;
use crate::presentation::http::agent::agent;
use crate::presentation::http::healthcheck::healthcheck;
use crate::presentation::state::app_state::AppState;
use axum::Router;
use axum::routing::{get, post};
use dotenvy::dotenv_override;
use std::sync::Arc;
use tokio::net::TcpListener;

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    dotenv_override().ok();

    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let llm_provider: Arc<dyn LlmProvider> = Arc::new(OpenaiLlmProvider::new(api_key));
    let agent_usecase = Arc::new(AgentUsecase::new(llm_provider));
    let app_state = AppState { agent_usecase };

    let routes = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/agent", post(agent));

    let app = Router::new().nest("/v1", routes).with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind listener");

    axum::serve(listener, app).await.expect("server error");
}
