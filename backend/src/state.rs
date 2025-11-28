use crate::ai::conversation_memory::ConversationMemory;
use crate::ai::socratic_engine::SocraticEngine;
use crate::game::components::{
    PeteCommandInbox, PeteResponseOutbox, ResearchLog, SharedPhysicsResource, VirtueTopology,
};
use leptos::config::LeptosOptions;
use sqlx::PgPool;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: Option<PgPool>,
    pub shared_research_log: Arc<RwLock<ResearchLog>>,
    pub shared_virtues: Arc<RwLock<VirtueTopology>>,
    pub gemma_server: Arc<crate::ai::llm::gemma_server::Gemma27BServer>,
    pub conversation_memory: Arc<ConversationMemory>,
    pub socratic_engine: Arc<tokio::sync::RwLock<SocraticEngine>>,
    pub model_manager: Arc<tokio::sync::Mutex<crate::services::model_manager::ModelManager>>,
    pub pete_assistant: Arc<crate::services::pete::PeteAssistant>,
    pub pete_command_inbox: PeteCommandInbox,
    pub pete_response_outbox: PeteResponseOutbox,
    pub shared_physics: SharedPhysicsResource,
}

impl axum::extract::FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

impl axum::extract::FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone().expect(
            "Database pool not available. This handler should not be reachable in simulation mode.",
        )
    }
}
