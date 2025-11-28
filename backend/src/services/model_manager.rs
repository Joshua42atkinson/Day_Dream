use anyhow::{Context, Result};
use hf_hub::api::sync::ApiBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Manages AI model downloads from HuggingFace and local caching
///
/// The ModelManager handles:
/// - Downloading models from HuggingFace Hub
/// - Caching models locally to avoid re-downloads
/// - Tracking which models are available
/// - Recommending models based on scenario complexity
pub struct ModelManager {
    cache_dir: PathBuf,
    downloaded_models: HashMap<String, ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub alias: String,
    pub size_mb: usize,
    pub path: Option<PathBuf>,
    pub description: String,
    pub recommended_for: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PluginComplexity {
    Simple,  // VaaM only, basic scenarios
    Medium,  // VaaM + Hero's Journey, standard narratives
    Complex, // Physics sims, AR, heavy compute
}

impl ModelManager {
    /// Create a new ModelManager with default cache directory
    pub fn new() -> Result<Self> {
        let cache_dir = Self::default_cache_dir()?;
        fs::create_dir_all(&cache_dir)?;

        Ok(Self {
            cache_dir,
            downloaded_models: HashMap::new(),
        })
    }

    /// Get the default cache directory for models
    /// Uses ~/.cache/askpeet/models on Unix-like systems
    /// Uses %LOCALAPPDATA%\askpeet\models on Windows
    fn default_cache_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;

        #[cfg(target_os = "windows")]
        let cache_dir = home
            .join("AppData")
            .join("Local")
            .join("askpeet")
            .join("models");

        #[cfg(not(target_os = "windows"))]
        let cache_dir = home.join(".cache").join("askpeet").join("models");

        Ok(cache_dir)
    }

    /// List all available models that can be downloaded
    pub fn list_available_models() -> Vec<ModelDefinition> {
        vec![
            ModelDefinition {
                id: "bartowski/gemma-2-2b-it-GGUF".to_string(),
                filename: "gemma-2-2b-it-Q4_K_M.gguf".to_string(),
                alias: "pete".to_string(),
                size_mb: 1500,
                description: "Lightweight model for Pete teacher assistant (Gemma 2 2B)"
                    .to_string(),
                required: true,
                recommended_for: vec!["authoring".to_string(), "assistance".to_string()],
            },
            ModelDefinition {
                id: "bartowski/gemma-2-9b-it-GGUF".to_string(),
                filename: "gemma-2-9b-it-Q4_K_M.gguf".to_string(),
                alias: "narrator".to_string(),
                size_mb: 6000,
                description: "Balanced model for student AI narrator (Gemma 2 9B)".to_string(),
                required: false,
                recommended_for: vec!["playing".to_string(), "narration".to_string()],
            },
            ModelDefinition {
                id: "bartowski/gemma-2-27b-it-GGUF".to_string(),
                filename: "gemma-2-27b-it-Q4_K_M.gguf".to_string(),
                alias: "advanced".to_string(),
                size_mb: 18000,
                description: "Powerful model for complex simulations (Gemma 2 27B)".to_string(),
                required: false,
                recommended_for: vec!["simulation".to_string(), "physics".to_string()],
            },
        ]
    }

    /// Download a model from HuggingFace
    pub async fn download_model(&mut self, model_def: &ModelDefinition) -> Result<PathBuf> {
        log::info!(
            "Downloading model: {} (alias: {})",
            model_def.id,
            model_def.alias
        );

        // Build HuggingFace API client
        let api = ApiBuilder::new()
            .with_cache_dir(self.cache_dir.clone())
            .build()?;

        // Get the model from HuggingFace
        let repo = api.model(model_def.id.clone());
        let filename = model_def.filename.clone();

        // Download the GGUF file (quantized model format)
        let model_path = tokio::task::spawn_blocking(move || repo.get(&filename)).await??;

        log::info!("Model downloaded to: {:?}", model_path);

        // Cache model info
        let model_info = ModelInfo {
            id: model_def.id.clone(),
            alias: model_def.alias.clone(),
            size_mb: model_def.size_mb,
            path: Some(model_path.clone()),
            description: model_def.description.clone(),
            recommended_for: model_def.recommended_for.clone(),
        };

        self.downloaded_models
            .insert(model_def.alias.clone(), model_info);

        Ok(model_path)
    }

    /// Check if a model is already downloaded
    pub fn has_model(&self, alias: &str) -> bool {
        self.downloaded_models
            .get(alias)
            .and_then(|info| info.path.as_ref())
            .map(|path| path.exists())
            .unwrap_or(false)
    }

    /// Get the path to a downloaded model
    pub fn get_model_path(&self, alias: &str) -> Option<&PathBuf> {
        self.downloaded_models
            .get(alias)
            .and_then(|info| info.path.as_ref())
    }

    /// Recommend a model based on plugin complexity
    pub fn recommend_model(&self, complexity: PluginComplexity) -> &str {
        match complexity {
            PluginComplexity::Simple => "pete",
            PluginComplexity::Medium => "narrator",
            PluginComplexity::Complex => "advanced",
        }
    }

    /// Calculate plugin complexity based on scenario configuration
    pub fn analyze_complexity(&self, scenario: &ScenarioConfig) -> PluginComplexity {
        let mut complexity_score = 0;

        // VaaM only: +0
        // VaaM + Framework: +1
        if scenario.framework.is_some() {
            complexity_score += 1;
        }

        // Custom node types (physics, AR, etc): +2 each
        complexity_score += scenario.custom_node_types.len() * 2;

        // Many assessments: +1
        if scenario.assessments.len() > 2 {
            complexity_score += 1;
        }

        match complexity_score {
            0..=1 => PluginComplexity::Simple,
            2..=3 => PluginComplexity::Medium,
            _ => PluginComplexity::Complex,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDefinition {
    pub id: String,
    pub filename: String,
    pub alias: String,
    pub size_mb: usize,
    pub description: String,
    pub required: bool,
    pub recommended_for: Vec<String>,
}

/// Temporary scenario config structure
/// TODO: Move to domain module when quest system is implemented
#[derive(Debug, Clone)]
pub struct ScenarioConfig {
    pub framework: Option<String>,
    pub assessments: Vec<String>,
    pub custom_node_types: Vec<String>,
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new().expect("Failed to create ModelManager")
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_cache_dir() {
        let cache_dir = ModelManager::default_cache_dir().unwrap();
        assert!(cache_dir.to_string_lossy().contains("askpeet"));
    }

    #[test]
    fn test_complexity_analysis() {
        let manager = ModelManager::new().unwrap();

        // Simple: VaaM only
        let simple = ScenarioConfig {
            framework: None,
            assessments: vec!["vaam".to_string()],
            custom_node_types: vec![],
        };
        assert_eq!(
            manager.analyze_complexity(&simple),
            PluginComplexity::Simple
        );

        // Medium: VaaM + Hero's Journey
        let medium = ScenarioConfig {
            framework: Some("heros_journey".to_string()),
            assessments: vec!["vaam".to_string()],
            custom_node_types: vec![],
        };
        assert_eq!(
            manager.analyze_complexity(&medium),
            PluginComplexity::Medium
        );

        // Complex: Physics sim
        let complex = ScenarioConfig {
            framework: Some("heros_journey".to_string()),
            assessments: vec!["vaam".to_string(), "physics".to_string()],
            custom_node_types: vec!["physics_sim".to_string(), "ar_node".to_string()],
        };
        assert_eq!(
            manager.analyze_complexity(&complex),
            PluginComplexity::Complex
        );
    }

    #[test]
    fn test_model_recommendation() {
        let manager = ModelManager::new().unwrap();

        assert_eq!(manager.recommend_model(PluginComplexity::Simple), "pete");
        assert_eq!(
            manager.recommend_model(PluginComplexity::Medium),
            "narrator"
        );
        assert_eq!(
            manager.recommend_model(PluginComplexity::Complex),
            "advanced"
        );
    }
}
*/
