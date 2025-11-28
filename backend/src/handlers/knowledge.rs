// Knowledge Base Management API
// Handles document upload, chunking, and embedding for RAG with Gemma 27B

use crate::error::AppError;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UploadKnowledgeRequest {
    pub title: String,
    pub content: String,
    pub source_type: String, // "pdf", "txt", "md"
}

#[derive(Debug, Serialize)]
pub struct UploadKnowledgeResponse {
    pub source_id: Uuid,
    pub chunks_created: usize,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub chunk_text: String,
    pub similarity: f32,
    pub source_title: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<usize>,
}

/// Upload a document and process it into searchable chunks
pub async fn upload_knowledge(
    Json(req): Json<UploadKnowledgeRequest>,
) -> Result<Json<UploadKnowledgeResponse>, AppError> {
    // TODO Phase 2: Implement document storage and chunking
    // 1. Store document in knowledge_sources table
    // 2. Chunk the content (500 tokens, 50 token overlap)
    // 3. Generate embeddings using fastembed
    // 4. Store vectors in knowledge_vectors table

    let source_id = Uuid::new_v4();

    Ok(Json(UploadKnowledgeResponse {
        source_id,
        chunks_created: 0,
        message: "Knowledge upload endpoint ready (implementation pending database setup)"
            .to_string(),
    }))
}

/// Search for relevant knowledge chunks using vector similarity
/// Uses Gemma 27B's synthesize_from_rag() for long-context orchestration
pub async fn search_knowledge(
    Json(req): Json<SearchRequest>,
) -> Result<Json<Vec<SearchResult>>, AppError> {
    // TODO Phase 2: Implement vector search with Gemma 27B synthesis
    // 1. Generate embedding for query using FastEmbed
    // 2. Perform cosine similarity search using pgvector
    // 3. Use Gemma 27B's synthesize_from_rag() to create coherent answer (8K context!)
    // 4. Return synthesized response with source attribution

    log::info!(
        "RAG search query: {} (limit: {})",
        req.query,
        req.limit.unwrap_or(5)
    );

    // Placeholder: In production, call:
    // let sources = vector_search(&req.query, req.limit.unwrap_or(5)).await?;
    // let synthesized = gemma_server.synthesize_from_rag(&req.query, &sources, 10)?;

    Ok(Json(vec![]))
}

/// Utility: Chunk text using sliding window approach
/// TODO: Upgrade to semantic chunking (sentence boundaries)
pub fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut chunks = Vec::new();

    if words.is_empty() {
        return chunks;
    }

    let mut i = 0;
    while i < words.len() {
        let end = (i + chunk_size).min(words.len());
        let chunk = words[i..end].join(" ");
        chunks.push(chunk);

        // Move forward by (chunk_size - overlap) to create overlap
        i += if chunk_size > overlap {
            chunk_size - overlap
        } else {
            chunk_size
        };
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_text() {
        let text = "This is a test sentence with multiple words for chunking.";
        let chunks = chunk_text(text, 3, 1);

        assert!(!chunks.is_empty());
        assert_eq!(chunks[0], "This is a");
        assert_eq!(chunks[1], "a test sentence");
    }
}
