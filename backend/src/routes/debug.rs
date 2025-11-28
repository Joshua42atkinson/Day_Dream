use crate::domain::railway::WordDefinition;
use crate::services::weigh_station::WeighStation;
use crate::state::AppState;
use axum::{extract::Query, response::Json, routing::get, Router};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeighParams {
    word: String,
}

pub fn debug_routes() -> Router<AppState> {
    Router::new().route("/weigh", get(weigh_word))
}

async fn weigh_word(Query(params): Query<WeighParams>) -> Json<WordDefinition> {
    let definition = WeighStation::weigh_cargo(&params.word);
    Json(definition)
}
