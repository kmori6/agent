use crate::application::usecase::agent_usecase::AgentUsecase;
use crate::domain::port::llm_provider::LlmProvider;
use crate::domain::port::tool::Tool;
use crate::infrastructure::llm_provider::openai_llm_provider::OpenaiLlmProvider;
use crate::infrastructure::tool::web_search_tool::WebSearchTool;
use crate::presentation::http::agent::agent;
use crate::presentation::http::healthcheck::healthcheck;
use crate::presentation::state::app_state::AppState;
use axum::Router;
use axum::routing::{get, post};
use dotenvy::dotenv_override;
use reqwest::Client;
use std::sync::Arc;
use tokio::net::TcpListener;

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {
    dotenv_override().ok();
    env_logger::init();

    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let tavily_api_key = std::env::var("TAVILY_API_KEY").expect("TAVILY_API_KEY must be set");

    let llm_provider: Arc<dyn LlmProvider> =
        Arc::new(OpenaiLlmProvider::new(openai_api_key, Client::new()));
    let tools: Vec<Arc<dyn Tool>> =
        vec![Arc::new(WebSearchTool::new(tavily_api_key, Client::new()))];
    let agent_usecase = Arc::new(AgentUsecase::new(llm_provider, tools));

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
