-- rollback makes updated_at inaccurate but preserves SeaORM type integrity
ALTER TABLE message ADD COLUMN updated_at timestamp with time zone NOT NULL DEFAULT now();
ALTER TABLE messageattempt ADD COLUMN updated_at timestamp with time zone NOT NULL DEFAULT now();
