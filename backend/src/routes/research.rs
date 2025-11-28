use crate::handlers::research::{get_research_log, log_research_event};
use crate::handlers::telemetry::log_telemetry;
use crate::services::notebook_lm::export_notebook_lm;
use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn research_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/api/research/logs", get(get_research_log))
        .route("/api/research/log", post(log_research_event))
        // New Interface A Routes
        .route("/api/telemetry", post(log_telemetry))
        .route("/api/research/export/notebooklm", get(export_notebook_lm))
        .with_state(state.clone())
}
