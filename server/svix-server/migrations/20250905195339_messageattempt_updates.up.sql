ALTER TABLE messageattempt DROP CONSTRAINT fk_messageattempt_msg_dest_id_messagedestination;
ALTER TABLE messageattempt ALTER COLUMN msg_dest_id DROP NOT NULL;

ALTER TABLE messageattempt ADD COLUMN IF NOT EXISTS attempt_number SMALLINT NOT NULL DEFAULT -1;

ALTER TABLE messageattempt ADD COLUMN IF NOT EXISTS next_attempt timestamptz NULL;
