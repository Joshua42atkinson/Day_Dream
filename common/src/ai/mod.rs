use serde::{Deserialize, Serialize};

/// Configuration for the Gemma 3 Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GemmaConfig {
    pub model_id: String, // e.g., "google/gemma-3-2b-it"
    pub quantized: bool,
    pub max_tokens: usize,
    pub temperature: f32,
}

impl Default for GemmaConfig {
    fn default() -> Self {
        Self {
            model_id: "google/gemma-3-2b-it".to_string(),
            quantized: true,
            max_tokens: 1024,
            temperature: 0.7,
        }
    }
}

/// Shared trait for AI Inference Engines.
/// Implemented by backend (Candle) and frontend (WASM/WebGPU).
pub trait InferenceEngine {
    fn generate(&self, prompt: &str) -> String;
    fn weigh_cargo(&self, text: &str) -> f32; // Returns cognitive weight
}
