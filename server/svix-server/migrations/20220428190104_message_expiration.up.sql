ALTER TABLE message ADD COLUMN expiration timestamp with time zone;
ALTER TABLE message ALTER COLUMN payload DROP NOT NULL;
CREATE INDEX message_payload_not_null_pidx ON message (payload) WHERE payload IS NOT NULL;
