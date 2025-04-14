-- Add response_duration_ms column to messageattempt table
ALTER TABLE messageattempt ADD COLUMN response_duration_ms BIGINT NOT NULL DEFAULT 0;
