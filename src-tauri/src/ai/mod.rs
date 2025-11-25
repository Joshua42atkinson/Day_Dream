use anyhow::{Error, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::llama::{Cache, Config, Llama};
use std::path::Path;

pub struct SocraticAgent {
    model: Llama,
    tokenizer: tokenizers::Tokenizer,
    device: Device,
}

impl SocraticAgent {
    pub fn new<P: AsRef<Path>>(model_path: P, tokenizer_path: P) -> Result<Self> {
        let device = Device::Cpu; // Android usually runs on CPU for now with Candle
        let config: Config =
            serde_json::from_slice(&std::fs::read(model_path.as_ref().join("config.json"))?)?;

        // Load the model (simplified for MVP, assuming safetensors)
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(
                &[model_path.as_ref().join("model.safetensors")],
                DType::F32,
                &device,
            )?
        };
        let model = Llama::load(vb, &config)?;
        let tokenizer = tokenizers::Tokenizer::from_file(tokenizer_path).map_err(Error::msg)?;

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    pub fn generate_response(&mut self, user_input: &str) -> Result<String> {
        let prompt = format!(
            "<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n\
            You are a Socratic Tutor. The user will share a thought. Ask one brief, reflective question to help them explore the underlying cause.<|eot_id|>\
            <|start_header_id|>user<|end_header_id|>\n\n\
            {}<|eot_id|>\
            <|start_header_id|>assistant<|end_header_id|>\n\n",
            user_input
        );

        let tokens = self.tokenizer.encode(prompt, true).map_err(Error::msg)?;
        let mut tokens = tokens.get_ids().to_vec();
        let mut cache = Cache::new(true, DType::F32, &self.model.config, &self.device)?;
        let mut logits_processor = LogitsProcessor::new(299792458, Some(0.6), Some(0.9)); // Seed, Temp, Top-P

        let mut response = String::new();

        for _ in 0..100 {
            // Max 100 tokens
            let input = Tensor::new(tokens.as_slice(), &self.device)?.unsqueeze(0)?;
            let logits = self.model.forward(&input, tokens.len(), &mut cache)?;
            let logits = logits.squeeze(0)?.squeeze(0)?.to_dtype(DType::F32)?;

            let next_token = logits_processor.sample(&logits)?;
            tokens.push(next_token);

            if let Some(text) = self.tokenizer.decode(&[next_token], true).ok() {
                response.push_str(&text);
                if text.contains("<|eot_id|>") {
                    break;
                }
            }
        }

        Ok(response.replace("<|eot_id|>", "").trim().to_string())
    }
}
