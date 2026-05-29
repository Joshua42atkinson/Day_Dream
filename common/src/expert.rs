use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct StoryChoice {
    pub id: String,
    pub label: String,
    pub description: String,
    pub leads_to: String,              // ID of the destination StoryNode
    pub pitch_gate: Option<f32>,       // PLING! pitch gate frequency in Hz
    #[serde(default)]
    pub virtue: Option<String>,         // Pedagogical virtue tag for trail tracking
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct StoryNode {
    pub id: String,
    pub title: String,
    pub content: String,
    pub x: f64,
    pub y: f64,
    pub subject_word: String,          // Active vocabulary word (VAAM)
    pub image_url: Option<String>,      // Ambient slide backdrop path
    pub audio_url: Option<String>,      // Backing audio track/song path
    pub target_freq: Option<f32>,       // Somatic pitch reference tone (Hz)
    #[serde(default)]
    pub choices: Vec<StoryChoice>,     // Branching choices for this node
    #[serde(default)]
    pub channel: Option<String>,        // MIND | HEART | BODY | ACTION
    #[serde(default)]
    pub depth: Option<String>,           // Socratic reflection question (double-tap)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Connection {
    pub id: String,
    pub from_node: String,
    pub to_node: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct StoryGraph {
    pub id: String,
    pub title: String,
    pub nodes: Vec<StoryNode>,
    pub connections: Vec<Connection>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub age_range: Option<String>,
}

// --- Trinity ID AI OS Types ---
// These types define the meta-pedagogical layers of the experience.

/// The Perspective Engineering Aesthetic Research Layout (Session Zero)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Pearl {
    pub subject: String,          // e.g. "Biology", "Cognitive Bias"
    pub vision: String,           // e.g. "Sci-Fi Survival", "LitRPG Fantasy"
}

/// The 12-phase pedagogical state machine mapped to the Hero's Journey
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum AddiecrapeyePhase {
    #[default]
    Analysis,      // 1. The Ordinary World
    Design,        // 2.
    Development,   // 3.
    Implementation,// 4.
    Evaluation,    // 5.
    Contrast,      // 6. The Ordeal
    Repetition,    // 7.
    Alignment,     // 8.
    Proximity,     // 9.
    Envision,      // 10.
    Yoke,          // 11.
    Evolve,        // 12. The Return (Reflection)
}

// --- Bevy ECS Bridge Types ---
// These types define the wire protocol between the frontend and the
// Bevy ECS virtue engine. When a student makes a choice, the frontend
// sends a ChoiceAction; the backend maps it to virtue adjustments and
// returns the updated VirtueSnapshot.

/// Sent by the frontend when a student selects a branching choice.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChoiceAction {
    pub graph_id: String,         // Which adventure graph
    pub node_id: String,          // Which slide they were on
    pub choice_id: String,        // Which choice they selected
    pub subject_word: String,     // The VAAM anchor word (e.g. "Bias", "Presence")
    pub leads_to: String,         // Destination node ID
}

/// Returned by the backend after processing a choice through Bevy ECS.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct VirtueSnapshot {
    pub self_efficacy: f32,
    pub self_esteem: f32,
    pub interdependence: f32,
    pub compassion: f32,
    pub valor: f32,
    pub inquiry: f32,        // Mapped from competence — reflects curiosity-driven learning
    pub resilience: f32,     // Mapped from honor — reflects persistence through difficulty
    pub presence: f32,       // Mapped from spirituality — reflects somatic grounding
    pub total_choices: u32,  // Running count of decisions made
}

