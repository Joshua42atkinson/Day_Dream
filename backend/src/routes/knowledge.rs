// Knowledge Base / RAG Routes
use crate::handlers::knowledge::{search_knowledge, upload_knowledge};
use crate::state::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn knowledge_routes() -> Router<AppState> {
    Router::new()
        .route("/api/knowledge/upload", post(upload_knowledge))
        .route("/api/knowledge/search", post(search_knowledge))
}
