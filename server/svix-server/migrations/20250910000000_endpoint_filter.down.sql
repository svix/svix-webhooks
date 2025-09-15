-- Drop filter column from endpoint table
ALTER TABLE endpoint
    DROP COLUMN IF EXISTS filter;


