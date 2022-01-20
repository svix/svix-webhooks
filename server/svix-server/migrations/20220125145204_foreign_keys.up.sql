-- Add up migration script here
-- For performance reasons we only add foreign keys for the "direct tree" relations (so partent -> child)
-- So we don't have to check too many constraints on items we insert a lot

ALTER TABLE endpoint ADD CONSTRAINT fk_endpoint_app_id_application FOREIGN KEY(app_id) REFERENCES application (id) ON DELETE CASCADE;
ALTER TABLE message ADD CONSTRAINT fk_message_app_id_application FOREIGN KEY(app_id) REFERENCES application (id) ON DELETE CASCADE;
ALTER TABLE messageattempt ADD CONSTRAINT fk_messageattempt_msg_dest_id_messagedestination FOREIGN KEY(msg_dest_id) REFERENCES messagedestination (id) ON DELETE CASCADE;
ALTER TABLE messagedestination ADD CONSTRAINT fk_messagedestination_msg_id_message FOREIGN KEY(msg_id) REFERENCES message (id) ON DELETE CASCADE;
ALTER TABLE messagedestination ADD CONSTRAINT fk_messagedestination_endp_id_endpoint FOREIGN KEY(endp_id) REFERENCES endpoint (id) ON DELETE CASCADE;
