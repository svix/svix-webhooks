CREATE TABLE application (
    id character varying NOT NULL COLLATE pg_catalog."C",
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    org_id character varying NOT NULL COLLATE pg_catalog."C",
    uid character varying COLLATE pg_catalog."C",
    name character varying NOT NULL,
    rate_limit integer,
    deleted boolean NOT NULL
);

ALTER TABLE ONLY application
    ADD CONSTRAINT pk_application PRIMARY KEY (id);

CREATE UNIQUE INDEX ix_application_unique_uid_per_org_cond ON application USING btree (org_id, uid) WHERE (uid IS NOT NULL);

CREATE TABLE endpoint (
    id character varying NOT NULL COLLATE pg_catalog."C",
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    app_id character varying NOT NULL COLLATE pg_catalog."C",
    key bytea NOT NULL,
    url character varying NOT NULL,
    description character varying NOT NULL,
    event_types_ids jsonb,
    version integer NOT NULL,
    rate_limit integer,
    deleted boolean NOT NULL,
    disabled boolean NOT NULL,
    first_failure_at timestamp with time zone,
    uid character varying COLLATE pg_catalog."C",
    old_keys jsonb,
    channels jsonb
);

ALTER TABLE ONLY endpoint
    ADD CONSTRAINT pk_endpoint PRIMARY KEY (id);

CREATE UNIQUE INDEX ix_endpoint_uid_unique_app_cond ON endpoint USING btree (app_id, uid) WHERE (uid IS NOT NULL);

CREATE TABLE eventtype (
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    id character varying NOT NULL COLLATE pg_catalog."C",
    org_id character varying NOT NULL COLLATE pg_catalog."C",
    description character varying NOT NULL,
    deleted boolean NOT NULL,
    schemas json,
    name character varying NOT NULL
);

ALTER TABLE ONLY eventtype
    ADD CONSTRAINT pk_eventtype PRIMARY KEY (id);

CREATE UNIQUE INDEX ix_event_type_unique_org ON eventtype USING btree (org_id, name);

CREATE TABLE message (
    id character varying NOT NULL COLLATE pg_catalog."C",
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    org_id character varying NOT NULL COLLATE pg_catalog."C",
    app_id character varying NOT NULL COLLATE pg_catalog."C",
    event_type character varying NOT NULL,
    uid character varying,
    payload jsonb NOT NULL,
    channels jsonb
);

ALTER TABLE ONLY message
    ADD CONSTRAINT pk_message PRIMARY KEY (id);

CREATE INDEX ix_message_per_app ON message USING btree (app_id, id DESC);

CREATE UNIQUE INDEX ix_message_uid_unique_app_cond ON message USING btree (app_id, uid) WHERE (uid IS NOT NULL);


CREATE TABLE messageattempt (
    id character varying NOT NULL COLLATE pg_catalog."C",
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    msg_id character varying NOT NULL COLLATE pg_catalog."C",
    msg_dest_id character varying NOT NULL COLLATE pg_catalog."C",
    endp_id character varying NOT NULL COLLATE pg_catalog."C",
    url character varying NOT NULL,
    status smallint NOT NULL,
    response_status_code smallint NOT NULL,
    response text DEFAULT ''::text NOT NULL,
    ended_at timestamp with time zone,
    trigger_type smallint NOT NULL
);

ALTER TABLE ONLY messageattempt
    ADD CONSTRAINT pk_messageattempt PRIMARY KEY (id);

CREATE INDEX ix_messageattempt_per_msg_no_status ON messageattempt USING btree (msg_id, id DESC);


CREATE TABLE messagedestination (
    id character varying NOT NULL COLLATE pg_catalog."C",
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL,
    msg_id character varying NOT NULL COLLATE pg_catalog."C",
    endp_id character varying NOT NULL COLLATE pg_catalog."C",
    status smallint NOT NULL,
    next_attempt timestamp with time zone
);

ALTER TABLE ONLY messagedestination
    ADD CONSTRAINT pk_messagedestination PRIMARY KEY (id);

CREATE INDEX ix_messagedestination_per_endp_no_status ON messagedestination USING btree (endp_id, id DESC);

CREATE INDEX ix_messagedestination_per_endp_with_status ON messagedestination USING btree (endp_id, status, id DESC);
