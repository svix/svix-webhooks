CREATE TABLE messagecontent (
    id character varying NOT NULL COLLATE pg_catalog."C",
    created_at timestamp with time zone NOT NULL,
    payload bytea NOT NULL,
    expiration timestamp with time zone NOT NULL DEFAULT now() + INTERVAL '90 day' 
);

ALTER TABLE ONLY messagecontent
    ADD CONSTRAINT pk_messagecontent PRIMARY KEY (id);

CREATE INDEX messagecontent_expiry ON messagecontent (expiration);
