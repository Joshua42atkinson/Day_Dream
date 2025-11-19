use axum::{
    routing::{get, post},
    Router,
};
use leptos::LeptosOptions;

use crate::handlers::vaam::{get_context_inventory, log_word_usage};

pub fn vaam_routes(_leptos_options: &LeptosOptions) -> Router {
    Router::new()
        .route("/api/vaam/context/:tag", get(get_context_inventory))
        .route("/api/vaam/log", post(log_word_usage))
}
