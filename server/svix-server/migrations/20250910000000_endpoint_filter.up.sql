-- Add optional filter column to endpoint table for payload-level filtering expressions
ALTER TABLE endpoint
    ADD COLUMN IF NOT EXISTS filter TEXT NULL;


