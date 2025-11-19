use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use crate::{
    domain::vaam::{VaamService, VocabWord, WordUsageRequest},
    Result,
};

/// GET /api/vaam/context/:tag
/// Returns the "Lexical Inventory" for a specific scene.
pub async fn get_context_inventory(
    State(pool): State<PgPool>,
    Path(tag): Path<String>,
) -> Result<Json<Vec<VocabWord>>> {
    let words = VaamService::get_words_for_context(&pool, &tag).await?;
    Ok(Json(words))
}

/// POST /api/vaam/log
/// The game client calls this when a player chooses a dialogue option.
/// Returns: true if the player just achieved mastery, false otherwise.
pub async fn log_word_usage(
    State(pool): State<PgPool>,
    Json(payload): Json<WordUsageRequest>,
) -> Result<Json<bool>> {
    let mastered_just_now = VaamService::log_usage(&pool, payload).await?;
    Ok(Json(mastered_just_now))
}
