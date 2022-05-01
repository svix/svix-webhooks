ALTER TABLE message ADD COLUMN expiration timestamp with time zone;
ALTER TABLE message ALTER COLUMN payload DROP NOT NULL;