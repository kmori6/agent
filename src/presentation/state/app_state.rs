use crate::application::usecase::agent_usecase::AgentUsecase;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub agent_usecase: Arc<AgentUsecase>,
}
