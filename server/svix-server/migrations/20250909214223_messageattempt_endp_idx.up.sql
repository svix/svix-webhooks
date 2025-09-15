CREATE INDEX IF NOT EXISTS messageattempt_per_endp_unified ON messageattempt (endp_id, status, id DESC) INCLUDE (response_status_code);
