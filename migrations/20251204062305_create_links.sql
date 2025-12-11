-- Add migration script here
CREATE TABLE IF NOT EXISTS urls (
    id BIGSERIAL PRIMARY KEY,
    short_code TEXT UNIQUE NOT NULL,
    original_url TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    hits BIGINT DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_short_code ON urls(short_code);
