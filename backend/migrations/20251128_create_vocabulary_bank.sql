-- backend/migrations/20251128_create_vocabulary_bank.sql

-- 1. The Depot: All available words and their physics
CREATE TABLE vocabulary_bank (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    word TEXT NOT NULL UNIQUE,
    definition TEXT NOT NULL,
    
    -- The "Physics" of the word
    grade_level INTEGER NOT NULL,       -- 0-12
    tier_level INTEGER NOT NULL,        -- 1 (Basic), 2 (Academic), 3 (Domain)
    cognitive_weight INTEGER NOT NULL,  -- 1-100 (The "Mass")
    
    -- "Gardening" Context
    domain_tags TEXT[],                 -- e.g. ["Science", "Abstract", "Verb"]
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 2. The Train Car: A container for a specific lesson
CREATE TABLE train_cars (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    node_id TEXT NOT NULL, -- Linked to the Graph (using TEXT for now)
    
    learning_objective TEXT NOT NULL,
    max_capacity INTEGER DEFAULT 100,    -- If cargo > 100, Train won't move
    current_load INTEGER DEFAULT 0,      -- Calculated sum of cargo weights
    
    is_locked BOOLEAN DEFAULT TRUE       -- Unlocks when load is "Mastered"
);

-- 3. The Cargo Manifest: Linking words to cars
CREATE TABLE car_cargo (
    car_id UUID REFERENCES train_cars(id) ON DELETE CASCADE,
    vocab_id UUID REFERENCES vocabulary_bank(id) ON DELETE CASCADE,
    
    is_mastered BOOLEAN DEFAULT FALSE,   -- When TRUE, weight drops to 0
    PRIMARY KEY (car_id, vocab_id)
);
