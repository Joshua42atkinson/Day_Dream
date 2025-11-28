use candle_core::Device;
use candle_transformers::models::quantized_llama::ModelWeights as QModel;
use candle_transformers::generation::LogitsProcessor;
// use tokenizers::Tokenizer; // Replaced by custom tokenizer
use crate::ai::tokenizer::GemmaTokenizer;
use wasm_bindgen::prelude::*;
use gloo_net::http::Request;

#[wasm_bindgen]
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
        web_sys::console::log_1(&"Booting up Gemma Agent...".into());

        // 1. Setup Device (Fallback to CPU for now due to wgpu dependency issues)
        let device = Device::Cpu;

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
        let _final_prompt = format!(
            "<start_of_turn>user\nCONTEXT: {}\n\nUSER SAYS: {}\n<end_of_turn>\n<start_of_turn>model\n", 
            node_prompt, 
            student_input
        );

        // ... Inference logic here (Encode -> Forward -> Decode) ...
        // Placeholder for brevity:
        "The Goblin looks at you with confusion. 'What is... photosynthesis?'".to_string()
    }
}

// Helper for fetching binary data
async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue> {
    let resp = Request::get(url).send().await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let bytes = resp.binary().await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(bytes)
}
