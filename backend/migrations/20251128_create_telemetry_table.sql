-- Create Telemetry Table for "Big Data" Lake
CREATE TABLE IF NOT EXISTS telemetry_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    event_type TEXT NOT NULL, -- "COAL_BURN", "STEAM_GAIN", "NODE_COMPLETE"
    value REAL NOT NULL,
    context TEXT NOT NULL, -- JSON string
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Index for faster querying by user and event type
CREATE INDEX idx_telemetry_user_event ON telemetry_logs(user_id, event_type);
