use crate::game::components::PhysicsState;
use crate::state::AppState;
use axum::{extract::State, Json};

pub async fn get_simulation_state(State(state): State<AppState>) -> Json<PhysicsState> {
    let physics = state.shared_physics.0.read().unwrap();
    Json(physics.clone())
}
