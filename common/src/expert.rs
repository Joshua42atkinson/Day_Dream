use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StoryNode {
    pub id: String,
    pub title: String,
    pub content: String,
    pub x: f64,
    pub y: f64,
    // Train Yard Metaphor Fields
    #[serde(default)]
    pub passenger_count: u8, // # of concepts introduced (Cognitive Load)
    #[serde(default)]
    pub complexity_level: u8, // 1-3 difficulty
    #[serde(default)]
    pub learner_profiles: Vec<String>, // Which "trains" can use this
    #[serde(default)]
    pub gardens_active: Vec<String>, // Which activities (Knowledge, Skills, Community)

    // Game State Logic (Triggers)
    #[serde(default)]
    pub required_stats: std::collections::HashMap<String, u32>, // e.g. "Strength" -> 5
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Connection {
    pub id: String,
    pub from_node: String,
    pub to_node: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StoryGraph {
    pub id: String,
    pub title: String,
    pub nodes: Vec<StoryNode>,
    pub connections: Vec<Connection>,
}
