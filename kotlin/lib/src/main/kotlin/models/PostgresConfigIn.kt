// This file is @generated
package com.svix.kotlin.models

import kotlinx.serialization.Serializable

@Serializable
data class PostgresConfigIn(
    /**
     * PostgreSQL connection URL, e.g. `postgres://user@host:5432/dbname?sslmode=require`.
     *
     * Do NOT embed a password here; use the `password` field instead.
     */
    val url: String,
    /** Password for the connection. */
    val password: String? = null,
    /**
     * Table to insert into. May be schema-qualified (e.g. `public.events`).
     *
     * Quote characters are not supported. Each dot-separated segment is automatically double-quoted
     * when the query is built, so `public.events` becomes `"public"."events"`.
     */
    val tableName: String,
    /**
     * PEM-encoded CA certificate used to verify the Postgres server's TLS certificate.
     *
     * Supply this to trust a private or self-signed CA when connecting with `sslmode=verify-ca` or
     * `sslmode=verify-full`. Without it, only the built-in public roots are trusted.
     */
    val sslRootCert: String? = null,
)
