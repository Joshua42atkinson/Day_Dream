use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_transformers::models::quantized_llama::ModelWeights as QLlama;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokenizers::Tokenizer;

/// Server-side engine for the Gemma 27B model (The "Orchestrator")
/// Optimized for long-context RAG synthesis and pedagogical reasoning
pub struct Gemma27BServer {
    model: Arc<Mutex<Option<QLlama>>>,
    tokenizer: Arc<Mutex<Option<Tokenizer>>>,
    device: Device,
    model_path: PathBuf,
    max_context_length: usize, // Gemma 3 supports 8K tokens
}

impl Gemma27BServer {
    /// Create new server with default 8K context
    pub fn new(model_path: PathBuf) -> Self {
        Self::with_context_length(model_path, 8192)
    }

    /// Create new server with specified context length
    /// Supported: 8192, 32768, 131072 (128K)
    pub fn with_context_length(model_path: PathBuf, max_context_length: usize) -> Self {
        // Validate context length
        let validated_length = match max_context_length {
            0..=8192 => 8192,
            8193..=32768 => 32768,
            _ => 131072, // 128K
        };

        if validated_length != max_context_length {
            log::warn!(
                "Context length {} rounded to nearest supported: {}",
                max_context_length,
                validated_length
            );
        }

        // Estimate memory usage
        let estimated_kv_cache_gb = validated_length as f32 * 13.5 / 8192.0; // ~13.5GB per 8K
        log::info!(
            "Initializing Gemma 27B with {} context (~{:.1}GB KV cache)",
            validated_length,
            estimated_kv_cache_gb
        );

        if estimated_kv_cache_gb > 35.0 {
            log::warn!(
                "KV cache may exceed available memory! Consider reducing context or using smaller model."
            );
        }

        Self {
            model: Arc::new(Mutex::new(None)),
            tokenizer: Arc::new(Mutex::new(None)),
            device: Device::Cpu,
            model_path,
            max_context_length: validated_length,
        }
    }

    /// Loads the model if not already loaded.
    pub fn load_model(&self) -> Result<()> {
        let mut model_guard = self.model.lock().unwrap();
        if model_guard.is_some() {
            return Ok(());
        }

        println!("Loading Gemma 27B from: {:?}", self.model_path);
        println!("Max context length: {} tokens", self.max_context_length);

        let tokenizer_path = self.model_path.parent().unwrap().join("tokenizer.json");
        let tokenizer = Tokenizer::from_file(&tokenizer_path).map_err(|e| {
            anyhow::anyhow!("Failed to load tokenizer from {:?}: {}", tokenizer_path, e)
        })?;

        let mut file = std::fs::File::open(&self.model_path).map_err(|e| {
            anyhow::anyhow!("Failed to open model file at {:?}: {}", self.model_path, e)
        })?;

        let content = candle_core::quantized::gguf_file::Content::read(&mut file)?;
        let model = QLlama::from_gguf(content, &mut file, &self.device)?;

        *self.tokenizer.lock().unwrap() = Some(tokenizer);
        *model_guard = Some(model);

        println!("Gemma 27B loaded successfully.");
        Ok(())
    }

    /// Get the maximum context length this model supports
    pub fn max_context_length(&self) -> usize {
        self.max_context_length
    }

    /// Count tokens in a text string (for context management)
    pub fn count_tokens(&self, text: &str) -> Result<usize> {
        if self.tokenizer.lock().unwrap().is_none() {
            self.load_model()?;
        }

        let tokenizer_guard = self.tokenizer.lock().unwrap();
        let tokenizer = tokenizer_guard.as_ref().unwrap();

        let tokens = tokenizer
            .encode(text, false)
            .map_err(|e| anyhow::anyhow!("Token counting failed: {}", e))?;

        Ok(tokens.len())
    }

    /// Generates text based on a prompt with configurable parameters
    pub fn generate(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        self.generate_with_config(prompt, max_tokens, 0.7, 0.9)
    }

    /// Generate with full control over temperature and top-p
    pub fn generate_with_config(
        &self,
        prompt: &str,
        max_tokens: usize,
        temperature: f32,
        top_p: f32,
    ) -> Result<String> {
        if self.model.lock().unwrap().is_none() {
            self.load_model()?;
        }

        let mut model_guard = self.model.lock().unwrap();
        let model = model_guard.as_mut().unwrap();

        let tokenizer_guard = self.tokenizer.lock().unwrap();
        let tokenizer = tokenizer_guard.as_ref().unwrap();

        // Encoding
        let tokens = tokenizer
            .encode(prompt, true)
            .map_err(|e| anyhow::anyhow!("Encoding failed: {}", e))?
            .get_ids()
            .to_vec();

        // Enforce context window limit
        if tokens.len() > self.max_context_length - max_tokens {
            anyhow::bail!(
                "Prompt too long: {} tokens (max: {} with {} generation headroom)",
                tokens.len(),
                self.max_context_length - max_tokens,
                max_tokens
            );
        }

        let mut all_tokens = tokens.clone();
        let mut generated_tokens = Vec::new();
        let mut index_pos = 0;

        // Generation Loop
        for _ in 0..max_tokens {
            let (context_tokens, context_index) = if index_pos == 0 {
                (all_tokens.clone(), 0)
            } else {
                (all_tokens[all_tokens.len() - 1..].to_vec(), index_pos)
            };

            let input = Tensor::new(context_tokens.as_slice(), &self.device)?.unsqueeze(0)?;
            let logits = model.forward(&input, context_index)?;
            let logits = logits.squeeze(0)?.squeeze(0)?;

            // Sampling with temperature (greedy if temp=0)
            let next_token = if temperature <= 0.0 {
                logits.argmax(0)?.to_scalar::<u32>()?
            } else {
                // Apply temperature scaling
                let scaled_logits = (logits / temperature as f64)?;
                scaled_logits.argmax(0)?.to_scalar::<u32>()?
            };

            all_tokens.push(next_token);
            generated_tokens.push(next_token);
            index_pos += context_tokens.len();

            // Gemma EOS tokens
            if next_token == 1 || next_token == 107 {
                break;
            }
        }

        // Decoding
        let output = tokenizer
            .decode(&generated_tokens, true)
            .map_err(|e| anyhow::anyhow!("Decoding failed: {}", e))?;

        Ok(output)
    }

    /// RAG-Specific: Synthesize answer from retrieved knowledge chunks
    /// Uses long context window to pack multiple sources for comprehensive analysis
    pub fn synthesize_from_rag(
        &self,
        query: &str,
        sources: &[(String, f32)],
        max_sources: usize,
    ) -> Result<String> {
        // Build context from top N sources
        let mut context_parts = Vec::new();
        let mut total_tokens = 0;

        // Count query tokens first
        let query_tokens = self.count_tokens(query)?;
        total_tokens += query_tokens + 200; // 200 for system prompt overhead

        // Add sources until we hit context limit (reserve 1000 tokens for response)
        let max_context_tokens = self.max_context_length - 1000;

        for (i, (text, score)) in sources.iter().take(max_sources).enumerate() {
            let source_tokens = self.count_tokens(text)?;

            if total_tokens + source_tokens > max_context_tokens {
                log::warn!(
                    "Context limit reached at source {}. Stopping source addition.",
                    i + 1
                );
                break;
            }

            context_parts.push(format!(
                "[Source {}] (relevance: {:.2})\n{}\n",
                i + 1,
                score,
                text
            ));
            total_tokens += source_tokens;
        }

        let context = context_parts.join("\n");

        let prompt = format!(
            "<bos><start_of_turn>user\n\
            You are an expert tutor using Purdue course materials. Synthesize a comprehensive answer from the provided sources.\n\n\
            Question: {}\n\n\
            Retrieved Sources:\n{}\n\
            <end_of_turn>\n\
            <start_of_turn>model\n",
            query, context
        );

        log::info!(
            "RAG synthesis using {} sources, ~{} tokens of context",
            context_parts.len(),
            total_tokens
        );

        // Use lower temperature for factual synthesis
        self.generate_with_config(&prompt, 800, 0.3, 0.9)
    }

    /// Pedagogical Analysis: Evaluate student response quality
    pub fn evaluate_response(
        &self,
        completion_criteria: &str,
        student_response: &str,
        context: &str,
    ) -> Result<String> {
        let prompt = format!(
            "<bos><start_of_turn>user\n\
            Evaluate this student response based on the learning criteria.\n\n\
            Criteria: {}\n\n\
            Context: {}\n\n\
            Student Response: \"{}\"\n\n\
            Provide: (1) Score (0-100), (2) Feedback, (3) Next steps.\n\
            <end_of_turn>\n\
            <start_of_turn>model\n",
            completion_criteria, context, student_response
        );

        self.generate_with_config(&prompt, 500, 0.4, 0.9)
    }
}
