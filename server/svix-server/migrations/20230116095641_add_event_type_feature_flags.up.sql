-- Add up migration script here
ALTER TABLE ONLY eventtype ADD COLUMN feature_flag text;
