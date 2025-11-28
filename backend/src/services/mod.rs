//! Services module for ASK PEET
//!
//! Contains standalone services that aren't tied to specific features:
//! Services module for ASK PETE
//!
//! Contains standalone services that aren't tied to specific features:
//! - Model Manager: Downloads and caches AI models from HuggingFace
//! - Pete: AI teacher assistant using RAG (Retrieval-Augmented Generation)

pub mod downloader;
pub mod model_manager;
pub mod model_registry; // [NEW]
pub mod pete; // [NEW]
pub mod weigh_station;
