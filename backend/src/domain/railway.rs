use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VocabularyTier {
    Tier1, // Basic (e.g., "Dog")
    Tier2, // Academic (e.g., "Analyze")
    Tier3, // Domain-Specific (e.g., "Isomorphism")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordDefinition {
    pub word: String,
    pub weight: u8, // Intrinsic Load (1-100)
    pub tier: VocabularyTier,
    pub embedding: Vec<f32>, // Semantic Vector
                             // pub dual_coding_assets: AssetHandle, // Placeholder for future asset integration
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainCar {
    pub id: Uuid,
    pub learning_objective: String,

    // The Physics of the Car
    pub max_cognitive_capacity: u8, // e.g., 100
    pub current_load: u8,           // Sum of cargo weights

    // The Cargo Manifest
    pub cargo: Vec<WordDefinition>,

    // The Coupling
    pub prerequisite_car_id: Option<Uuid>,

    // The Door Lock (Unlock Condition)
    pub mastery_threshold: f32, // e.g., 0.8 (80%)
}

impl TrainCar {
    pub fn new(objective: String, capacity: u8) -> Self {
        Self {
            id: Uuid::new_v4(),
            learning_objective: objective,
            max_cognitive_capacity: capacity,
            current_load: 0,
            cargo: Vec::new(),
            prerequisite_car_id: None,
            mastery_threshold: 0.8,
        }
    }

    pub fn add_cargo(&mut self, word: WordDefinition) -> Result<(), String> {
        if self.current_load + word.weight > self.max_cognitive_capacity {
            return Err(format!(
                "Cargo Overload: Adding '{}' (Weight: {}) would exceed capacity ({}/{})",
                word.word, word.weight, self.current_load, self.max_cognitive_capacity
            ));
        }
        self.current_load += word.weight;
        self.cargo.push(word);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_train_car_capacity() {
        let mut car = TrainCar::new("Test Objective".to_string(), 10);

        let light_word = WordDefinition {
            word: "Cat".to_string(),
            weight: 5,
            tier: VocabularyTier::Tier1,
            embedding: vec![],
        };

        let heavy_word = WordDefinition {
            word: "Photosynthesis".to_string(),
            weight: 50,
            tier: VocabularyTier::Tier3,
            embedding: vec![],
        };

        // Should succeed
        assert!(car.add_cargo(light_word.clone()).is_ok());
        assert_eq!(car.current_load, 5);

        // Should fail (5 + 50 > 10)
        assert!(car.add_cargo(heavy_word).is_err());
        assert_eq!(car.current_load, 5); // Load should not change
    }
}
