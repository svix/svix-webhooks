BEGIN;
ALTER TABLE message DROP COLUMN expiration;
ALTER TABLE message ALTER COLUMN payload SET NOT NULL;
DROP INDEX message_payload_idx;
COMMIT;