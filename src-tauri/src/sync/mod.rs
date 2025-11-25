use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct XpUpdate {
    pub amount: i32,
    pub reason: String,
}

pub struct SyncService {
    client: Client,
    backend_url: String,
}

impl SyncService {
    pub fn new(backend_url: String) -> Self {
        Self {
            client: Client::new(),
            backend_url,
        }
    }

    pub async fn push_xp(&self, amount: i32, reason: &str) -> Result<()> {
        let update = XpUpdate {
            amount,
            reason: reason.to_string(),
        };

        let res = self
            .client
            .post(format!("{}/api/player/xp", self.backend_url))
            .json(&update)
            .send()
            .await?;

        if !res.status().is_success() {
            return Err(anyhow::anyhow!("Failed to sync XP: {}", res.status()));
        }

        Ok(())
    }
}
