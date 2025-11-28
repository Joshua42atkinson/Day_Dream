use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use uuid::Uuid;

/// A complete narrative graph authored in the canvas
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NarrativeGraph {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    #[sqlx(json)]
    pub graph_data: GraphData,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// The actual graph structure (nodes + edges)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<StoryNodeData>,
    pub edges: Vec<StoryEdge>,
    pub start_node_id: String,
}

/// A node in the narrative graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryNodeData {
    pub id: String,
    pub node_type: NodeType,
    pub position: Position,
    pub content: NodeContent,

    // --- The Antigravity Updates ---
    /// The "Stage Directions" for the Client-Side AI (Gemma)
    #[serde(default)]
    pub context_prompt: String,

    /// The "Grading Rubric" for the Server-Side AI (Llama)
    #[serde(default)]
    pub completion_criteria: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeType {
    #[serde(rename = "story")]
    Story,
    #[serde(rename = "choice")]
    Choice,
    #[serde(rename = "condition")]
    Condition,
    #[serde(rename = "effect")]
    Effect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeContent {
    pub text: String,
    pub speaker: Option<String>,
    pub choices: Vec<ChoiceOption>,
    pub effects: HashMap<String, EffectValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceOption {
    pub id: String,
    pub text: String,
    pub required_virtues: Option<HashMap<String, f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EffectValue {
    Number(f32),
    Text(String),
    Boolean(bool),
}

/// An edge connecting two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryEdge {
    pub from_node_id: String,
    pub to_node_id: String,
    pub choice_id: Option<String>, // Which choice leads to this edge
}

/// Request body for creating a new narrative
#[derive(Debug, Deserialize)]
pub struct CreateNarrativeRequest {
    pub title: String,
    pub description: Option<String>,
    pub graph_data: GraphData,
}

/// Request body for updating a narrative
#[derive(Debug, Deserialize)]
pub struct UpdateNarrativeRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub graph_data: Option<GraphData>,
}
