use crate::{AppError, AppState, Result};
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Serialize)]
pub struct NotebookLMExport {
    pub report_title: String,
    pub generated_at: String,
    pub data: Vec<ExportRow>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ExportRow {
    pub user_id: String,
    pub event_type: String,
    pub total_value: f32,
    pub count: i64,
}

pub async fn export_notebook_lm(
    State(app_state): State<AppState>,
) -> Result<Json<NotebookLMExport>> {
    let pool = match app_state.pool {
        Some(pool) => pool,
        None => return Err(AppError::InternalServerError), // Can't export in sim mode
    };

    // Aggregation query for "Academic Impact Report"
    // Sums up values per user per event type
    let rows = sqlx::query_as::<_, ExportRow>(
        r#"
        SELECT 
            user_id, 
            event_type, 
            SUM(value) as total_value, 
            COUNT(*) as count
        FROM telemetry_logs
        GROUP BY user_id, event_type
        ORDER BY user_id, event_type
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {:?}", e);
        AppError::InternalServerError
    })?;

    let export = NotebookLMExport {
        report_title: "Ask Pete Academic Impact Report".to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        data: rows,
    };

    Ok(Json(export))
}
