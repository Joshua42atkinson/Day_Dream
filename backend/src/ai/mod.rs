pub mod conversation_memory;
pub mod llm;
pub mod prompts;
pub mod socratic_engine;

pub use conversation_memory::{ConversationMemory, Speaker, Turn, TurnMetadata};
pub use socratic_engine::{SessionContext, SocraticEngine, SocraticResponse};
