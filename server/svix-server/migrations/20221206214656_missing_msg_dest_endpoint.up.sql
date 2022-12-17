-- Add up migration script here
CREATE INDEX ix_messagedestination_per_msg_no_status ON messagedestination USING btree (msg_id);
