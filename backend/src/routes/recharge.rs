use crate::handlers::recharge::{get_department_report, report_miles};
use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub fn recharge_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/api/recharge/report", post(report_miles))
        .route("/api/recharge/department", get(get_department_report))
        .with_state(state.clone())
}
