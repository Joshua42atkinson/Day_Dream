use crate::handlers::simulation;
use crate::state::AppState;
use axum::{routing::get, Router};

pub fn simulation_routes() -> Router<AppState> {
    Router::new().route(
        "/api/simulation/state",
        get(simulation::get_simulation_state),
    )
}
