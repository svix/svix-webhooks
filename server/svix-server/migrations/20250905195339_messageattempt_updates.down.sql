ALTER TABLE messageattempt ADD CONSTRAINT fk_messageattempt_msg_dest_id_messagedestination FOREIGN KEY(msg_dest_id) REFERENCES messagedestination (id) ON DELETE CASCADE;
ALTER TABLE messageattempt ALTER COLUMN msg_dest_id SET NOT NULL;

ALTER TABLE messageattempt DROP COLUMN IF EXISTS attempt_number;

ALTER TABLE messageattempt DROP COLUMN IF EXISTS next_attempt;
