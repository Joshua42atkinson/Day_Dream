use axum::{
    routing::{get, post},
    Router,
};
use leptos::LeptosOptions;

use crate::handlers::persona::{get_dilemmas, get_archetypes, submit_quiz};

pub fn persona_routes(leptos_options: &LeptosOptions) -> Router<LeptosOptions> {
    Router::new()
        .route("/api/dilemmas", get(get_dilemmas))
        .route("/api/archetypes", get(get_archetypes))
        .route("/api/submit_quiz", post(submit_quiz))
        .with_state(leptos_options.clone())
}
