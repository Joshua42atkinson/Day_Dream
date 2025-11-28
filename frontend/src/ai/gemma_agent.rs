use candle_core::Device;
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::quantized_llama::ModelWeights as QModel;
// use tokenizers::Tokenizer; // Replaced by custom tokenizer
use crate::ai::tokenizer::GemmaTokenizer;
use gloo_net::http::Request;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(dead_code)]
pub struct GemmaAgent {
    model: QModel,
    tokenizer: GemmaTokenizer,
    device: Device,
    logits_processor: LogitsProcessor,
}

#[wasm_bindgen]
impl GemmaAgent {
    /// Initialize the Agent by downloading the GGUF model and Tokenizer
    /// directly into the browser's memory (WebGPU).
    pub async fn new(model_url: &str, tokenizer_url: &str) -> Result<GemmaAgent, JsValue> {
        web_sys::console::log_1(&"Booting up Gemma Agent (WebGPU)...".into());

        // 1. Setup Device (Try WebGPU, fallback to CPU)
        let device = Device::new_cuda(0).unwrap_or(Device::Cpu); // Placeholder for actual WebGPU init
        // Note: Actual WebGPU support in Candle WASM requires specific feature flags and setup.
        // For this phase, we'll stick to CPU/WASM execution to ensure compatibility,
        // as full WebGPU support in Candle is still experimental.
        // We will simulate "Coal" consumption here.

        // 2. Fetch Resources
        let _model_data = fetch_bytes(model_url).await?;
        let tokenizer_data = fetch_bytes(tokenizer_url).await?;

        // 3. Reconstruct Model
        let mut cursor = std::io::Cursor::new(_model_data);
        let content = candle_core::quantized::gguf_file::Content::read(&mut cursor)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let model = QModel::from_gguf(content, &mut cursor, &device)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let tokenizer_json = String::from_utf8(tokenizer_data)
            .map_err(|e| JsValue::from_str(&format!("Invalid UTF-8: {}", e)))?;

        let tokenizer = GemmaTokenizer::from_json_str(&tokenizer_json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(GemmaAgent {
            model,
            tokenizer,
            device,
            logits_processor: LogitsProcessor::new(299792458, None, None),
        })
    }

    /// The "Act" Phase: Generate text based on the Node's context.
    pub fn generate_response(&mut self, node_prompt: &str, student_input: &str) -> String {
        // Construct the prompt with the "Master Prompt" from the Node
        let final_prompt = format!(
            "<start_of_turn>user\nCONTEXT: {}\n\nUSER SAYS: {}\n<end_of_turn>\n<start_of_turn>model\n", 
            node_prompt, 
            student_input
        );

        // Tokenize
        let tokens = match self.tokenizer.encode(&final_prompt) {
            Ok(t) => t,
            Err(_) => return "Error: Tokenization failed.".to_string(),
        };

        // Inference Loop (Simplified for WASM)
        // In a real WASM environment, this would be async and stream tokens back.
        // For now, we return a placeholder to prove the agent is "alive".
        
        // Calculate Coal Cost using shared logic
        let cost = common::economy::Coal::cost_local(tokens.len());
        web_sys::console::log_1(&format!("(Gemma 3) Burning Coal: {:.4} for {} tokens", cost.0, tokens.len()).into());

        let response = format!("(Gemma 3 Local): I heard you say '{}'. I am processing this locally on your device.", student_input);
        
        response
    }
}

// Helper for fetching binary data
async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue> {
    let resp = Request::get(url)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let bytes = resp
        .binary()
        .await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(bytes)
}
