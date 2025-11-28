use serde::{Deserialize, Serialize};

/// Schema for the Local Vector Store (LanceDB).
/// Stores journal entries and their embeddings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: String,
    pub timestamp: i64,
    pub content: String,
    pub sentiment_score: f32,
    pub embedding: Vec<f32>, // Vector embedding from Gemma
    pub tags: Vec<String>,
}

/// Schema for Telemetry Logs (Coal/Steam).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryLog {
    pub user_id: String,
    pub timestamp: i64,
    pub event_type: String, // "COAL_BURN", "STEAM_GAIN", "NODE_COMPLETE"
    pub value: f32,
    pub context: String, // JSON string with details
}
