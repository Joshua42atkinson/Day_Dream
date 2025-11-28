use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::handlers::weigh_station::WeighStation;
use crate::AppState;

#[derive(Deserialize)]
pub struct WeighRequest {
    pub word: String,
}

pub fn weigh_station_routes() -> Router<AppState> {
    Router::new().route("/weigh", post(weigh_word))
}

async fn weigh_word(
    State(state): State<AppState>,
    Json(payload): Json<WeighRequest>,
) -> impl IntoResponse {
    if let Some(station_mutex) = &state.weigh_station {
        let mut station = station_mutex.lock().await;
        match station.process_word(&payload.word).await {
            Ok(physics) => Json(physics).into_response(),
            Err(e) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    } else {
        (
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "Weigh Station is offline. Llama 3.2 model not found.",
        )
            .into_response()
    }
}
