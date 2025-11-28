use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a "Lesson" or "Node" in the curriculum.
/// Metaphor: A car added to the train.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainCar {
    pub id: String,
    pub title: String,
    pub description: String,
    pub cargo: Vec<Cargo>,   // Vocabulary words in this lesson
    pub weight: f32,         // Total cognitive load of this car
    pub required_steam: f32, // Mastery needed to attach this car
}

/// Represents a "Vocabulary Word" or "Concept".
/// Metaphor: The freight being carried.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cargo {
    pub id: String,
    pub term: String,
    pub definition: String,
    pub weight: f32, // Cognitive load (1-100)
    pub tier: u8,    // 1, 2, or 3
}

/// Represents the learner's current status.
/// Metaphor: The Locomotive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentState {
    pub id: String,
    pub coal: f32,              // Current mental energy (0-100)
    pub steam: f32,             // Current mastery/currency
    pub velocity: f32,          // Learning momentum
    pub engine_power: f32,      // Baseline capacity (willpower/IQ/grit)
    pub location: (f64, f64),   // GPS Coordinates (for Physical AI)
    pub inventory: Vec<String>, // IDs of collected Cargo
}
