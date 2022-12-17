-- Add up migration script here
CREATE TABLE endpointmetadata (
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    id character varying NOT NULL COLLATE pg_catalog."C",
    data jsonb NOT NULL
);

ALTER TABLE ONLY endpointmetadata 
    ADD CONSTRAINT endpointmetadata_pkey PRIMARY KEY (id);