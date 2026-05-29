use crate::handlers::expert::{get_graph, save_graph, submit_choice};
use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn expert_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/api/expert/graph", get(get_graph).post(save_graph))
        .route("/api/quest/action", post(submit_choice))
}
