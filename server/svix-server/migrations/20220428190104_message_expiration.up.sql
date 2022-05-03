ALTER TABLE message ADD COLUMN expiration timestamp with time zone NOT NULL DEFAULT now() + INTERVAL '90 day';
ALTER TABLE message ALTER COLUMN payload DROP NOT NULL;
CREATE INDEX message_payload_not_null_pidx ON message (expiration) WHERE payload IS NOT NULL;
