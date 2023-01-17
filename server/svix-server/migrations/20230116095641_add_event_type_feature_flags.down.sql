-- Add down migration script here
ALTER TABLE ONLY eventtype DROP COLUMN feature_flag;
