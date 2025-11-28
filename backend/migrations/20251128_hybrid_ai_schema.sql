-- 1. Enable the pgvector extension to store AI Embeddings
CREATE EXTENSION IF NOT EXISTS vector;

-- 2. Add the "Director's Instructions" to the Node (Existing Logic)
-- Note: Since we are storing the graph as a JSON blob in 'narrative_graphs', 
-- these columns might not be needed on a separate 'story_nodes' table unless we are normalizing.
-- However, the user request explicitly asked for this SQL.
-- If 'story_nodes' table doesn't exist, we might need to create it or assume it's part of the JSON.
-- But the user's SQL implies ALTER TABLE. I will assume the table exists or this is for a normalized view.
-- If it fails, the user might be using the JSON storage model primarily.
-- But for now, I will follow the user's instruction.

-- Check if table exists before altering, or just run ALTER (Postgres will error if table missing).
-- Better to create if not exists or just run the ALTERs if we are sure.
-- Given the user's prompt, I'll stick to the requested SQL but wrap in a transaction or checks if possible.
-- For simplicity and to match the request, I will output the requested SQL.

ALTER TABLE story_nodes 
ADD COLUMN IF NOT EXISTS context_prompt TEXT NOT NULL DEFAULT 'You are a helpful tutor.',
ADD COLUMN IF NOT EXISTS completion_criteria TEXT NOT NULL DEFAULT 'The user must demonstrate understanding.';

-- 3. Add "Mass" to the Vocabulary Table (Existing Logic)
ALTER TABLE vocabulary_terms
ADD COLUMN IF NOT EXISTS cognitive_weight INT NOT NULL DEFAULT 10; -- 1-100 scale

-- 4. Create the Knowledge Base (The "Library")
CREATE TABLE IF NOT EXISTS knowledge_sources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    content TEXT NOT NULL, -- The full text content
    source_type TEXT NOT NULL, -- 'pdf', 'txt', 'md'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 5. Create the Vector Store (The "NotebookLM" Brain)
CREATE TABLE IF NOT EXISTS knowledge_vectors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_id UUID REFERENCES knowledge_sources(id) ON DELETE CASCADE,
    chunk_text TEXT NOT NULL, -- The specific paragraph/sentence
    chunk_index INT NOT NULL, -- To reconstruct order if needed
    
    -- The Embedding Vector. 
    -- 768 is the standard dimension for 'nomic-embed-text' or 'bert-base'.
    -- The user mentioned 768.
    embedding vector(768) 
);

-- 6. Create an HNSW Index for fast semantic search (The "Recall" Mechanism)
CREATE INDEX IF NOT EXISTS knowledge_vectors_embedding_idx 
ON knowledge_vectors 
USING hnsw (embedding vector_cosine_ops);
