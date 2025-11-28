-- Add columns to vocabulary_words table to support Weigh Station
ALTER TABLE vocabulary_words 
ADD COLUMN weight INTEGER NOT NULL DEFAULT 1,
ADD COLUMN grade_level INTEGER NOT NULL DEFAULT 0,
ADD COLUMN tier INTEGER NOT NULL DEFAULT 1,
ADD COLUMN tags TEXT[] NOT NULL DEFAULT '{}';
