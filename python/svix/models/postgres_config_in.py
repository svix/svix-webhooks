# this file is @generated
import typing as t

from .common import BaseModel


class PostgresConfigIn(BaseModel):
    url: str
    """PostgreSQL connection URL, e.g. `postgres://user@host:5432/dbname?sslmode=require`.

    Do NOT embed a password here; use the `password` field instead."""

    password: t.Optional[str] = None
    """Password for the connection."""

    table_name: str
    """Table to insert into. May be schema-qualified (e.g. `public.events`).

    Quote characters are not supported. Each dot-separated segment is automatically double-quoted when the query is built, so `public.events` becomes `"public"."events"`."""

    ssl_root_cert: t.Optional[str] = None
    """PEM-encoded CA certificate used to verify the Postgres server's TLS certificate.

    Supply this to trust a private or self-signed CA when connecting with `sslmode=verify-ca` or `sslmode=verify-full`. Without it, only the built-in public roots are trusted."""
