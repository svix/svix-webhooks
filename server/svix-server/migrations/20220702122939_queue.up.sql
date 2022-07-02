CREATE TABLE IF NOT EXISTS queue(
    id SERIAL PRIMARY KEY,
    queue_name VARCHAR(50) NOT NULL,
    payload JSON NOT NULL,
    when_run TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL
);

CREATE INDEX queue_when ON queue(when_run);