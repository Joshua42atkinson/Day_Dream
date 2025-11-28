use crate::ai::llm::{GemmaModel, GenerationConfig};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

/// The "Ticket" Pete gives back after weighing a word
#[derive(Debug, Serialize, Deserialize)]
pub struct WordPhysics {
    pub word: String,
    pub definition: String,
    pub grade_level: i32,
    pub tier: i32,
    pub weight: i32, // 1-100
    pub tags: Vec<String>,
}

pub struct WeighStation {
    db: PgPool,
    llm: GemmaModel,
}

impl WeighStation {
    pub fn new(db: PgPool, llm: GemmaModel) -> Self {
        Self { db, llm }
    }

    /// The core loop: Takes a raw word, weighs it, stores it.
    pub async fn process_word(&mut self, raw_word: &str) -> Result<WordPhysics> {
        println!("⚖️  Weighing: '{}'...", raw_word);

        // 1. Construct the Prompt (The "Scale")
        let prompt = format!(
            r#"You are an Expert Instructional Designer and Linguist. 
            Analyze the English word: "{}" for a K-12 Curriculum.
            
            Return ONLY a JSON object with this exact structure:
            {{
                "word": "{}",
                "definition": "A simple, clear definition for a student.",
                "grade_level": <integer 0-12>,
                "tier": <integer 1, 2, or 3>,
                "weight": <integer 1-100 representing cognitive load difficulty>,
                "tags": ["<tag1>", "<tag2>"]
            }}
            
            RUBRIC:
            - Tier 1: Basic conversation (Weight 1-10)
            - Tier 2: Academic cross-curriculum (Weight 11-50)
            - Tier 3: Domain specific/Technical (Weight 51-100)
            "#,
            raw_word, raw_word
        );

        // 2. Ask Pete (The Inference)
        let config = GenerationConfig {
            max_tokens: 300,
            temperature: 0.2, // Low temp for precision/consistency
            ..Default::default()
        };

        let json_response = self.llm.generate(&prompt, config)?;

        // 3. Parse the Physics
        // Clean markdown code blocks if present
        let clean_json = json_response
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let physics: WordPhysics =
            serde_json::from_str(clean_json).context("Failed to parse Pete's weighing ticket")?;

        // 4. Store in the Depot
        self.store_in_depot(&physics).await?;

        println!("✅ Stored: {} (Weight: {})", physics.word, physics.weight);
        Ok(physics)
    }

    async fn store_in_depot(&self, p: &WordPhysics) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO vocabulary_words 
            (word, definition, grade_level, tier, weight, tags)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (word) DO UPDATE 
            SET weight = $5, -- Update weight if it changed
                definition = $2,
                grade_level = $3,
                tier = $4,
                tags = $6
            "#,
        )
        .bind(&p.word)
        .bind(&p.definition)
        .bind(p.grade_level)
        .bind(p.tier)
        .bind(p.weight)
        .bind(&p.tags)
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
