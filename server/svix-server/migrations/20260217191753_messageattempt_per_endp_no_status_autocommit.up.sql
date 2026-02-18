
SET statement_timeout = 0;

CREATE INDEX CONCURRENTLY IF NOT EXISTS messageattempt_per_endp_no_status ON messageattempt USING btree (endp_id, id DESC) INCLUDE (status, response_status_code);

RESET statement_timeout;
