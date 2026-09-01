// Package svix this file is @generated DO NOT EDIT
package models

type PostgresConfigIn struct {
	// PostgreSQL connection URL, e.g. `postgres://user@host:5432/dbname?sslmode=require`.
	//
	// Do NOT embed a password here; use the `password` field instead.
	Url      string  `json:"url"`
	Password *string `json:"password,omitempty"` // Password for the connection.
	// Table to insert into. May be schema-qualified (e.g. `public.events`).
	//
	// Quote characters are not supported. Each dot-separated segment is automatically double-quoted when the query is built, so `public.events` becomes `"public"."events"`.
	TableName string `json:"tableName"`
	// PEM-encoded CA certificate used to verify the Postgres server's TLS certificate.
	//
	// Supply this to trust a private or self-signed CA when connecting with `sslmode=verify-ca` or `sslmode=verify-full`. Without it, only the built-in public roots are trusted.
	SslRootCert *string `json:"sslRootCert,omitempty"`
}
