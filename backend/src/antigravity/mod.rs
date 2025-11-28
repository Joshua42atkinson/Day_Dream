use anyhow::Result;
use common::economy::Steam;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

/// Client for the Antigravity Enterprise Backend
pub struct AntigravityClient {
    client: Client,
    base_url: String,
    api_key: String,
}

#[derive(Serialize)]
struct SteamSyncRequest {
    user_id: String,
    steam_amount: f64,
    source: String,
    timestamp: String,
}

#[derive(Deserialize)]
struct SteamSyncResponse {
    success: bool,
    new_balance: f64,
}

impl AntigravityClient {
    pub fn new() -> Self {
        let base_url = env::var("ANTIGRAVITY_URL")
            .unwrap_or_else(|_| "https://api.antigravity.dev".to_string());
        let api_key = env::var("ANTIGRAVITY_API_KEY").unwrap_or_default();

        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    /// Sync generated "Steam" to the cloud
    pub async fn sync_steam(&self, user_id: &str, steam: Steam, source: &str) -> Result<f64> {
        if self.api_key.is_empty() {
            log::warn!("ANTIGRAVITY_API_KEY not set. Skipping Steam sync.");
            return Ok(0.0);
        }

        let url = format!("{}/api/v1/steam/sync", self.base_url);
        let request = SteamSyncRequest {
            user_id: user_id.to_string(),
            steam_amount: steam.0,
            source: source.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            anyhow::bail!("Antigravity API Error: {}", error_text);
        }

        let sync_response: SteamSyncResponse = response.json().await?;
        Ok(sync_response.new_balance)
    }
}
