use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

/// Configuration for Gemini generation
#[derive(Debug, Clone)]
pub struct GeminiConfig {
    pub api_key: String,
    pub model: String,
    pub max_tokens: usize,
    pub temperature: f32,
}

impl Default for GeminiConfig {
    fn default() -> Self {
        Self {
            api_key: env::var("GEMINI_API_KEY").unwrap_or_default(),
            model: "gemini-1.5-pro-latest".to_string(), // Using 1.5 Pro as proxy for 3 Ultra for now
            max_tokens: 1024,
            temperature: 0.7,
        }
    }
}

/// Request payload for Gemini API
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    generationConfig: GenerationConfig,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    maxOutputTokens: usize,
    temperature: f32,
}

/// Response payload from Gemini API
#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<Candidate>>,
    error: Option<ErrorResponse>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentResponse,
    finishReason: Option<String>,
}

#[derive(Deserialize)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    code: i32,
    message: String,
    status: String,
}

/// Client for Google Gemini API
pub struct GeminiClient {
    client: Client,
    config: GeminiConfig,
    coal_balance: f64, // Track "Coal" usage
}

impl GeminiClient {
    pub fn new(config: GeminiConfig) -> Self {
        Self {
            client: Client::new(),
            config,
            coal_balance: 100.0, // Initial coal grant
        }
    }

    /// Generate text from a prompt
    pub async fn generate(&mut self, prompt: &str) -> Result<String> {
        if self.config.api_key.is_empty() {
            anyhow::bail!("GEMINI_API_KEY not set");
        }

        // 1. Calculate Coal Cost (Metaphor for Compute)
        let cost = common::economy::Coal::cost_cloud();
        if self.coal_balance < cost.0 {
            anyhow::bail!(
                "Insufficient Coal. Need {:.2}, have {:.2}. Burn more coal in the sandbox!",
                cost.0,
                self.coal_balance
            );
        }
        self.coal_balance -= cost.0;
        log::info!(
            "Burning Coal: -{:.2}. Remaining: {:.2}",
            cost.0,
            self.coal_balance
        );

        // 2. Prepare Request
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.config.model, self.config.api_key
        );

        let request_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
            generationConfig: GenerationConfig {
                maxOutputTokens: self.config.max_tokens,
                temperature: self.config.temperature,
            },
        };

        // 3. Send Request
        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to Gemini API")?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Gemini API Error: {}", error_text);
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .context("Failed to parse Gemini response")?;

        // 4. Handle Error in Body
        if let Some(err) = gemini_response.error {
            anyhow::bail!(
                "Gemini API returned error: {} ({})",
                err.message,
                err.status
            );
        }

        // 5. Extract Text
        if let Some(candidates) = gemini_response.candidates {
            if let Some(first_candidate) = candidates.first() {
                if let Some(first_part) = first_candidate.content.parts.first() {
                    return Ok(first_part.text.clone());
                }
            }
        }

        Ok("No response generated.".to_string())
    }

    /// Get current coal balance
    pub fn get_coal_balance(&self) -> f64 {
        self.coal_balance
    }
}
