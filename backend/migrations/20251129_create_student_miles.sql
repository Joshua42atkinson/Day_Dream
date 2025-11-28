CREATE TABLE IF NOT EXISTS student_miles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL,
    miles_earned FLOAT NOT NULL DEFAULT 0.0,
    miles_spent FLOAT NOT NULL DEFAULT 0.0,
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS mileage_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL,
    amount FLOAT NOT NULL,
    reason TEXT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
