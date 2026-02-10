CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    topic TEXT NOT NULL,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_topic ON events(topic);