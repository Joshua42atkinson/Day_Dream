use crate::services::model_manager::ModelDefinition;
use crate::AppState;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

pub fn pete_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/api/pete/models", get(list_models))
        .route("/api/pete/models/download", post(download_model))
        .route("/api/pete/chat", post(chat_with_pete))
        .with_state(state.clone())
}

async fn list_models(State(state): State<AppState>) -> impl IntoResponse {
    let manager = state.model_manager.lock().await;
    let models = crate::services::model_manager::ModelManager::list_available_models();

    // Enrich with status (downloaded or not)
    let enriched_models: Vec<EnrichedModelDefinition> = models
        .into_iter()
        .map(|m| {
            let downloaded = manager.has_model(&m.alias);
            EnrichedModelDefinition {
                definition: m,
                downloaded,
            }
        })
        .collect();

    Json(enriched_models)
}

#[derive(Deserialize)]
struct DownloadRequest {
    alias: String,
}

async fn download_model(
    State(state): State<AppState>,
    Json(payload): Json<DownloadRequest>,
) -> impl IntoResponse {
    let mut manager = state.model_manager.lock().await;
    let models = crate::services::model_manager::ModelManager::list_available_models();

    if let Some(model_def) = models.iter().find(|m| m.alias == payload.alias) {
        match manager.download_model(model_def).await {
            Ok(_) => {
                Json(serde_json::json!({ "status": "success", "message": "Model downloaded" }))
            }
            Err(e) => Json(serde_json::json!({ "status": "error", "message": e.to_string() })),
        }
    } else {
        Json(serde_json::json!({ "status": "error", "message": "Model not found" }))
    }
}

#[derive(Deserialize)]
struct ChatRequest {
    message: String,
}

use crate::game::components::{AskPeteEvent, PeteResponseEvent};

async fn chat_with_pete(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    // 1. Construct Event
    let ask_event = AskPeteEvent {
        content: payload.message.clone(),
        context: "Direct Chat".to_string(), // TODO: Get actual context if possible
    };

    // 2. Push to Bevy Inbox (so game knows about it)
    if let Ok(mut inbox) = state.pete_command_inbox.0.write() {
        inbox.push(ask_event);
    }

    // 3. Get Response from Socratic Engine
    // We need to lock the engine to use it
    let mut engine = state.socratic_engine.write().await;

    // Create a temporary session context
    // TODO: Retrieve actual session context from DB or Memory
    let context = crate::ai::socratic_engine::SessionContext {
        session_id: uuid::Uuid::new_v4(),
        user_id: 1, // Placeholder
        archetype: None,
        focus_area: Some("chat".to_string()),
    };

    match engine.respond(&payload.message, &context).await {
        Ok(response) => {
            // 4. Push Response to Bevy Outbox (so game knows about it)
            let response_event = PeteResponseEvent {
                content: response.text.clone(),
            };
            if let Ok(mut outbox) = state.pete_response_outbox.0.write() {
                outbox.push(response_event);
            }

            Json(serde_json::json!({ "status": "success", "data": response }))
        }
        Err(e) => Json(serde_json::json!({ "status": "error", "message": e.to_string() })),
    }
}

#[derive(Serialize)]
struct EnrichedModelDefinition {
    #[serde(flatten)]
    definition: ModelDefinition,
    downloaded: bool,
}
