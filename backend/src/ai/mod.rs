pub mod conversation_memory;
// pub mod embedding; // RAG embedding service
pub mod llm;
pub mod prompts;
pub mod socratic_engine;

#[cfg(test)]
// mod tests;
pub use socratic_engine::SessionContext;
