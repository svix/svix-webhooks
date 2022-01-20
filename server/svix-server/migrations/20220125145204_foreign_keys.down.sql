-- Add down migration script here

ALTER TABLE endpoint DROP CONSTRAINT fk_endpoint_app_id_application;
ALTER TABLE message DROP CONSTRAINT fk_message_app_id_application;
ALTER TABLE messageattempt DROP CONSTRAINT fk_messageattempt_msg_dest_id_messagedestination;
ALTER TABLE messagedestination DROP CONSTRAINT fk_messagedestination_msg_id_message;
ALTER TABLE messagedestination DROP CONSTRAINT fk_messagedestination_endp_id_endpoint;
