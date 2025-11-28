use crate::domain::railway::{TrainCar, VocabularyTier, WordDefinition};

pub struct WeighStation;

impl WeighStation {
    /// Calculates the "Intrinsic Load" (Weight) of a word based on heuristics.
    /// In a full implementation, this would use an LLM or frequency dictionary.
    pub fn weigh_cargo(word: &str) -> WordDefinition {
        let length = word.len();
        let (weight, tier) = match length {
            0..=4 => (5, VocabularyTier::Tier1), // Short words (e.g., "Cat")
            5..=8 => (20, VocabularyTier::Tier2), // Medium words (e.g., "Planet")
            _ => (50, VocabularyTier::Tier3),    // Long words (e.g., "Photosynthesis")
        };

        WordDefinition {
            word: word.to_string(),
            weight,
            tier,
            embedding: vec![], // Placeholder for actual embeddings
        }
    }

    /// Validates if a TrainCar is safe to depart (not overloaded).
    pub fn validate_car(car: &TrainCar) -> Result<bool, String> {
        if car.current_load > car.max_cognitive_capacity {
            return Err(format!(
                "Overload detected: {}/{}",
                car.current_load, car.max_cognitive_capacity
            ));
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weigh_cargo() {
        let w1 = WeighStation::weigh_cargo("Cat");
        assert_eq!(w1.weight, 5);
        assert_eq!(w1.tier, VocabularyTier::Tier1);

        let w2 = WeighStation::weigh_cargo("Planet");
        assert_eq!(w2.weight, 20);
        assert_eq!(w2.tier, VocabularyTier::Tier2);

        let w3 = WeighStation::weigh_cargo("Photosynthesis");
        assert_eq!(w3.weight, 50);
        assert_eq!(w3.tier, VocabularyTier::Tier3);
    }
}
