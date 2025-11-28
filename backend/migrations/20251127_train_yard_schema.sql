-- Train Yard Architecture Migration
-- Enables RAG (Retrieval-Augmented Generation) with vector similarity search
-- Created: 2025-11-27

-- 1. Enable pgvector extension for vector operations
CREATE EXTENSION IF NOT EXISTS vector;

-- 2. Add AI Stage Directions to story_nodes
-- These fields control client-side and server-side AI behavior
ALTER TABLE story_nodes 
ADD COLUMN IF NOT EXISTS context_prompt TEXT NOT NULL DEFAULT 'You are a helpful tutor guiding the student through this learning experience.',
ADD COLUMN IF NOT EXISTS completion_criteria TEXT NOT NULL DEFAULT 'The student must demonstrate understanding of the concept.';

COMMENT ON COLUMN story_nodes.context_prompt IS 'Instructions for client-side AI (Gemma 2B) - defines character/behavior for this node';
COMMENT ON COLUMN story_nodes.completion_criteria IS 'Grading rubric for server-side AI (Llama 3) - evaluates student progress';

-- 3. Add Cognitive Weight to vocabulary
-- Weight represents cognitive load (1-100 scale, higher = more demanding)
ALTER TABLE vocabulary_terms
ADD COLUMN IF NOT EXISTS cognitive_weight INT NOT NULL DEFAULT 10 CHECK (cognitive_weight >= 1 AND cognitive_weight <= 100);

COMMENT ON COLUMN vocabulary_terms.cognitive_weight IS 'Cognitive load score: 1-30 (light), 31-70 (moderate), 71-100 (heavy)';

-- 4. Create Knowledge Base table
-- Stores raw uploaded documents (PDFs, textbooks, markdown)
CREATE TABLE IF NOT EXISTS knowledge_sources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    source_type TEXT NOT NULL CHECK (source_type IN ('pdf', 'txt', 'md', 'html')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS knowledge_sources_created_at_idx ON knowledge_sources(created_at DESC);

COMMENT ON TABLE knowledge_sources IS 'Repository of instructor-uploaded course materials';

-- 5. Create Vector Store table
-- Stores chunked text with 768-dimensional embeddings for semantic search
CREATE TABLE IF NOT EXISTS knowledge_vectors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_id UUID NOT NULL REFERENCES knowledge_sources(id) ON DELETE CASCADE,
    chunk_text TEXT NOT NULL,
    chunk_index INT NOT NULL,
    embedding vector(768), -- Dimension for nomic-embed-text or all-MiniLM-L6-v2 (384)
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS knowledge_vectors_source_id_idx ON knowledge_vectors(source_id);

COMMENT ON TABLE knowledge_vectors IS 'Embedded text chunks for RAG semantic similarity search';
COMMENT ON COLUMN knowledge_vectors.embedding IS 'Text embedding vector (768-dim for nomic-embed-text, 384-dim for all-MiniLM-L6-v2)';

-- 6. Create HNSW index for fast vector similarity search
-- HNSW (Hierarchical Navigable Small World) is optimal for high-dimensional vectors
-- Uses cosine distance (vector_cosine_ops) for similarity
CREATE INDEX IF NOT EXISTS knowledge_vectors_embedding_idx 
ON knowledge_vectors 
USING hnsw (embedding vector_cosine_ops)
WITH (m = 16, ef_construction = 64); -- HNSW parameters tuned for 768-dim vectors

COMMENT ON INDEX knowledge_vectors_embedding_idx IS 'HNSW index for O(log n) vector similarity search';
