// this file is @generated
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PostgresConfigIn {
    /// PostgreSQL connection URL, e.g.
    /// `postgres://user@host:5432/dbname?sslmode=require`.
    ///
    /// Do NOT embed a password here; use the `password` field instead.
    pub url: String,

    /// Password for the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Table to insert into. May be schema-qualified (e.g. `public.events`).
    ///
    /// Quote characters are not supported. Each dot-separated segment is
    /// automatically double-quoted when the query is built, so `public.events`
    /// becomes `"public"."events"`.
    #[serde(rename = "tableName")]
    pub table_name: String,

    /// PEM-encoded CA certificate used to verify the Postgres server's TLS
    /// certificate.
    ///
    /// Supply this to trust a private or self-signed CA when connecting with
    /// `sslmode=verify-ca` or `sslmode=verify-full`. Without it, only the
    /// built-in public roots are trusted.
    #[serde(rename = "sslRootCert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_root_cert: Option<String>,
}

impl PostgresConfigIn {
    pub fn new(url: String, table_name: String) -> Self {
        Self {
            url,
            password: None,
            table_name,
            ssl_root_cert: None,
        }
    }
}
