use serde::{Deserialize, Serialize};

/// The Physics Engine for Learning.
/// Calculates how much "Coal" is burned by processing "Cargo".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveLoad;

impl CognitiveLoad {
    /// Calculates the load rating (0-100) for a given text complexity and student engine power.
    ///
    /// # Arguments
    /// * `cargo_weight` - The inherent difficulty of the concept (1-100).
    /// * `engine_power` - The student's current capacity (1-100).
    ///
    /// # Returns
    /// * `f32` - The actual load experienced by the student.
    pub fn calculate_load(cargo_weight: f32, engine_power: f32) -> f32 {
        // Basic Physics: Load = Weight / Power
        // If Weight > Power, Load spikes exponentially (Overload).

        let ratio = cargo_weight / engine_power.max(1.0); // Avoid div by zero

        if ratio > 1.0 {
            // Overload Zone: Exponential penalty
            (ratio.powf(1.5) * 50.0).min(100.0)
        } else {
            // Safe Zone: Linear load
            ratio * 50.0
        }
    }

    /// Calculates coal burn for a specific duration of study.
    pub fn calculate_burn(load_rating: f32, duration_seconds: f32) -> f32 {
        // Burn Rate = Load * Time
        // High load burns coal faster.
        let burn_rate_per_sec = load_rating / 1000.0;
        burn_rate_per_sec * duration_seconds
    }
}
