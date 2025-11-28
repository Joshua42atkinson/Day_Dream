use crate::AppState;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReportMilesRequest {
    pub student_id: uuid::Uuid,
    pub amount: f64,
    pub reason: String,
}

pub async fn report_miles(
    State(state): State<AppState>,
    Json(payload): Json<ReportMilesRequest>,
) -> impl IntoResponse {
    // In simulation mode (no DB), just log it
    if state.pool.is_none() {
        println!(
            "SIMULATION: Reporting {} miles for student {} because: {}",
            payload.amount, payload.student_id, payload.reason
        );
        return Json(serde_json::json!({ "status": "success", "message": "Simulated report" }));
    }

    let pool = state.pool.as_ref().unwrap();

    // TODO: Implement actual SQL transaction
    // For now, just placeholder
    Json(serde_json::json!({ "status": "success", "message": "Miles reported (DB placeholder)" }))
}

pub async fn get_department_report(State(_state): State<AppState>) -> impl IntoResponse {
    // Placeholder report
    let report = serde_json::json!({
        "department": "Engineering",
        "total_miles_earned": 1000.0,
        "total_miles_spent": 250.0,
        "balance": 750.0
    });
    Json(report)
}
