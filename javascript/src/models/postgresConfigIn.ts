// this file is @generated

export interface PostgresConfigIn {
  /**
   * PostgreSQL connection URL, e.g. `postgres://user@host:5432/dbname?sslmode=require`.
   *
   * Do NOT embed a password here; use the `password` field instead.
   */
  url: string;
  /** Password for the connection. */
  password?: string | null;
  /**
   * Table to insert into. May be schema-qualified (e.g. `public.events`).
   *
   * Quote characters are not supported. Each dot-separated segment is automatically double-quoted when the query is built, so `public.events` becomes `"public"."events"`.
   */
  tableName: string;
  /**
   * PEM-encoded CA certificate used to verify the Postgres server's TLS certificate.
   *
   * Supply this to trust a private or self-signed CA when connecting with `sslmode=verify-ca` or `sslmode=verify-full`. Without it, only the built-in public roots are trusted.
   */
  sslRootCert?: string | null;
}

export const PostgresConfigInSerializer = {
  _fromJsonObject(object: any): PostgresConfigIn {
    return {
      url: object["url"],
      password: object["password"],
      tableName: object["tableName"],
      sslRootCert: object["sslRootCert"],
    };
  },

  _toJsonObject(self: PostgresConfigIn): any {
    return {
      url: self.url,
      password: self.password,
      tableName: self.tableName,
      sslRootCert: self.sslRootCert,
    };
  },
};
