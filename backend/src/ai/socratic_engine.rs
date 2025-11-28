use super::conversation_memory::{ConversationMemory, Speaker, Turn};
use super::prompts::PromptStrategy;
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Response from the Socratic engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocraticResponse {
    pub text: String,
    pub strategy_used: PromptStrategy,
}

/// Context for the current session
pub struct SessionContext {
    pub session_id: Uuid,
    pub user_id: i64,
    pub archetype: Option<String>,
    pub focus_area: Option<String>,
}

/// Main Socratic dialogue engine
pub struct SocraticEngine {
    gemini_client: Option<crate::ai::llm::gemini_client::GeminiClient>,
    antigravity_client: Option<crate::antigravity::AntigravityClient>,
    memory: Arc<ConversationMemory>,
}

impl SocraticEngine {
    /// Create a new Socratic engine
    pub fn new(memory: Arc<ConversationMemory>) -> Self {
        Self {
            gemini_client: None,
            antigravity_client: None,
            memory,
        }
    }

    /// Set the Gemini client for LLM inference
    pub fn set_gemini_client(&mut self, client: crate::ai::llm::gemini_client::GeminiClient) {
        self.gemini_client = Some(client);
        log::info!("Gemini client connected to Socratic engine");
    }

    /// Set the Antigravity client for Steam sync
    pub fn set_antigravity_client(&mut self, client: crate::antigravity::AntigravityClient) {
        self.antigravity_client = Some(client);
        log::info!("Antigravity client connected to Socratic engine");
    }

    /// Generate a Socratic response to user input
    pub async fn respond(
        &mut self,
        user_input: &str,
        context: &SessionContext,
    ) -> Result<SocraticResponse> {
        log::debug!(
            "Generating Socratic response for session {}",
            context.session_id
        );

        // 1. Save user's turn to memory
        let user_turn = Turn {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            speaker: Speaker::User,
            content: user_input.to_string(),
            metadata: Default::default(),
        };
        self.memory.add_turn(context.session_id, user_turn).await?;

        // 2. Retrieve recent conversation history
        let history = self.memory.get_recent(context.session_id, 10).await?;
        log::debug!(
            "Retrieved {} turns from conversation history",
            history.len()
        );

        // 3. Select prompting strategy based on user input
        let strategy = PromptStrategy::select_strategy(user_input, &history);
        log::debug!("Selected strategy: {:?}", strategy);

        // 4. Build prompt with template
        let prompt = strategy.build_prompt(user_input, &history, context);
        log::debug!("Built prompt: {} chars", prompt.len());

        // 5. Generate response using LLM
        let response_text = if let Some(ref mut gemini_client) = self.gemini_client {
            // Actual inference using Gemini
            match gemini_client.generate(&prompt).await {
                Ok(text) => text,
                Err(e) => {
                    log::error!("Gemini generation failed: {}", e);
                    "I'm having trouble connecting to my thoughts (Gemini API Error).".to_string()
                }
            }
        } else {
            // Fallback if Gemini client not connected
            log::warn!("Gemini client not connected, using fallback response");
            "I'm listening. Can you tell me more about that?".to_string()
        };

        // 6. Post-process response
        let processed_response = Self::post_process_response(&response_text);

        // 7. Save AI's turn to memory
        let ai_turn = Turn {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            speaker: Speaker::AI,
            content: processed_response.clone(),
            metadata: Default::default(),
        };
        self.memory.add_turn(context.session_id, ai_turn).await?;

        // 8. Generate Steam (Mastery) & Sync to Antigravity
        // Simple heuristic: 1 Steam per successful turn
        let steam_earned = common::economy::Steam(1.0);
        if let Some(ref client) = self.antigravity_client {
            let user_id_str = context.user_id.to_string();
            // Fire and forget sync (don't block response)
            let _ = client
                .sync_steam(&user_id_str, steam_earned, "socratic_dialogue")
                .await;
            log::info!(
                "Generated Steam: {:.2} and syncing to Antigravity",
                steam_earned.0
            );
        }

        Ok(SocraticResponse {
            text: processed_response,
            strategy_used: strategy,
        })
    }

    /// Post-process AI response to ensure it's Socratic
    fn post_process_response(response: &str) -> String {
        let mut processed = response.trim().to_string();

        // Ensure response doesn't give direct answers (basic heuristic)
        // TODO: More sophisticated filtering

        // Ensure response ends with a question mark
        if !processed.ends_with('?') {
            if !processed.is_empty() {
                processed.push_str("?");
            }
        }

        processed
    }
}
