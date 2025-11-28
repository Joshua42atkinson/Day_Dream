// Embedding Service for RAG
// Generates text embeddings using fastembed (Rust-native)

use anyhow::Result;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::sync::{Arc, Mutex};

/// Thread-safe embedding service
#[derive(Clone)]
pub struct EmbeddingService {
    model: Arc<Mutex<TextEmbedding>>,
}

impl EmbeddingService {
    /// Initialize the embedding model
    /// Uses all-MiniLM-L6-v2 (384-dim) for balance of speed and quality
    pub async fn new() -> Result<Self> {
        log::info!("Initializing FastEmbed embedding model...");

        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
        )?;

        log::info!("Embedding model loaded successfully");

        Ok(Self {
            model: Arc::new(Mutex::new(model)),
        })
    }

    /// Generate embeddings for a batch of texts
    /// Returns vectors of dimension 384 (for AllMiniLML6V2)
    pub fn embed_batch(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>> {
        let mut model = self.model.lock().unwrap();
        let embeddings = model.embed(texts, None)?;
        Ok(embeddings)
    }

    /// Generate embedding for a single text
    pub fn embed_single(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.embed_batch(vec![text.to_string()])?;
        Ok(embeddings.into_iter().next().unwrap_or_default())
    }

    /// Get the dimension of embeddings produced by this model
    pub fn dimension(&self) -> usize {
        384 // AllMiniLML6V2 produces 384-dimensional vectors
    }
}

/// Vector similarity utilities
pub mod similarity {
    /// Cosine similarity between two vectors
    /// Returns value in [-1, 1], higher = more similar
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }

        dot_product / (norm_a * norm_b)
    }
}

#[cfg(test)]
mod tests {
    use super::similarity::*;
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let c = vec![0.0, 1.0, 0.0];

        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);
        assert!((cosine_similarity(&a, &c)).abs() < 0.001);
    }
}
