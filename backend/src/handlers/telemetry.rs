use crate::{AppError, AppState, Result};
use axum::{extract::State, Json};
use common::db::TelemetryLog;
use sqlx::PgPool;

pub async fn log_telemetry(
    State(app_state): State<AppState>,
    Json(payload): Json<TelemetryLog>,
) -> Result<Json<String>> {
    let pool = match app_state.pool {
        Some(pool) => pool,
        None => return Ok(Json("Simulation Mode: Logged".to_string())),
    };

    sqlx::query(
        r#"
        INSERT INTO telemetry_logs (user_id, timestamp, event_type, value, context)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&payload.user_id)
    .bind(payload.timestamp)
    .bind(&payload.event_type)
    .bind(payload.value)
    .bind(&payload.context)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        AppError::InternalServerError
    })?;

    Ok(Json("Logged".to_string()))
}
