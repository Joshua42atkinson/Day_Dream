#![allow(unused_imports)]
use super::conversation_memory::ConversationMemory;
use crate::ai::llm::llama_engine::{LlamaModel, MockLlamaModel};
use crate::ai::llm::{GenerationConfig, Llama3Model, ModelConfig};
use uuid::Uuid;

/*
#[test]
fn test_mock_llama_model() {
    let mut mock_model = MockLlamaModel {
        response: "Mock response".to_string(),
    };

    let config = GenerationConfig::default();
    let response = mock_model.generate("Test prompt", config).unwrap();

    assert_eq!(response, "Mock response");
}

#[tokio::test]
#[ignore]
async fn test_real_ai_generation() {
    // This test requires actual model files to be present
    let config = ModelConfig::default();

    // Check if model exists before running
    if !config.model_path.exists() {
        println!(
            "Skipping real AI test: Model not found at {:?}",
            config.model_path
        );
        return;
    }

    let mut model = Llama3Model::load(config).expect("Failed to load model");
    let gen_config = GenerationConfig::default();

    let prompt = "<|begin_of_text|><|start_header_id|>user<|end_header_id|>\n\nHello, who are you?<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n";

    let response = model
        .generate(prompt, gen_config)
        .expect("Failed to generate");

    println!("Generated response: {}", response);
    assert!(!response.is_empty());
}
*/
