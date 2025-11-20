-- Create reflection_entries table
CREATE TABLE reflection_entries (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    challenge_name VARCHAR(255) NOT NULL,
    reflection_text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
