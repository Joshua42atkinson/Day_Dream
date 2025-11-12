use axum::{
    routing::{get, post},
    Router,
};
use leptos::LeptosOptions;

use crate::handlers::player::{
    get_profile_data,
    get_player_character,
    get_journal_data,
    handle_submit_command,
};

pub fn player_routes(leptos_options: &LeptosOptions) -> Router<LeptosOptions> {
    Router::new()
        .route("/api/profile_data", get(get_profile_data))
        .route("/api/player_character", get(get_player_character))
        .route("/api/journal_data", get(get_journal_data))
        .route("/api/submit_command", post(handle_submit_command))
        .with_state(leptos_options.clone())
}
