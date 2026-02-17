use std::{
    sync::LazyLock,
    time::{Duration, Instant},
};

use futures_util::future::BoxFuture;
use regex::Regex;
use sqlx::{
    Executor,
    migrate::{AppliedMigration, Migrate, MigrateError, Migration},
    query,
};

static COMMENT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?m)^\\s*--.*").unwrap());
static SPLIT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?m);\\s*$").unwrap());

/// Takes semicolon `;` separated SQL statements and split them into a vec of distinct statements.
///
/// Also filters out comments.
///
/// This handles the case where `;` is in a string or comment (e.g. `WHERE id = 'bob;burger'`), but it should
/// only run on trusted sql we've validated ahead of time, as it's not a proper tokenizer.
fn split_trusted_sql_statements(sql: &str) -> Vec<String> {
    let sql = COMMENT_REGEX.replace_all(sql, "");

    SPLIT_REGEX
        .split(&sql)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
        .collect()
}

/// Prefix for 'autocommit' transactions.
///
/// Migrations that should run each line in an individual 'autocommit' transaction
/// (running each line via an separate 'execute' call) need to be suffixed with `autocommit`, e.g.
/// "20230316154602_add_index_concurrently_autocommit.up.sql".
/// This allows multiple 'CREATE INDEX CONCURRENTLY' statements in a single file - otherwise,
/// our single 'execute' call will create a transaction containing all of them, which will fail
/// due to 'CREATE INDEX CONCURRENTLY' being blocked inside transactions
pub const AUTOCOMMIT_SUFFIX: &str = "autocommit";

/// A custom sqlx::Migrator - used for running select migrations outside of transactions.
///
/// sqlx currently doesn't support running migrations out of txns natively - which
/// prevents things like CREATE INDEX CONCURRENTLY . . .
///
/// This let's us get around that.
pub struct TxnOptionalMigrator {
    conn: sqlx::PgConnection,
    name: &'static str,
}

impl TxnOptionalMigrator {
    pub async fn new(name: &'static str, p: &sqlx::PgPool) -> Self {
        // Removes a connection from the pool permanently. In practice, this limits the total # of max connections
        // p can support (if it was N, now it is N-1).
        //
        // THIS IS OKAY! Because We only initialize a pool to run migrations,
        // and then immediately drop the pool. The leaked connection has no
        // impact on our connection pool used to service clients.
        let conn = p
            .acquire()
            .await
            .unwrap_or_else(|_| panic!("Couldn't acquire connection to {name} from pool"))
            .leak();

        Self { conn, name }
    }
}

impl Migrate for TxnOptionalMigrator {
    fn ensure_migrations_table(&mut self) -> BoxFuture<'_, Result<(), MigrateError>> {
        self.conn.ensure_migrations_table()
    }

    fn dirty_version(&mut self) -> BoxFuture<'_, Result<Option<i64>, MigrateError>> {
        self.conn.dirty_version()
    }

    fn list_applied_migrations(
        &mut self,
    ) -> BoxFuture<'_, Result<Vec<AppliedMigration>, MigrateError>> {
        self.conn.list_applied_migrations()
    }

    fn lock(&mut self) -> BoxFuture<'_, Result<(), MigrateError>> {
        self.conn.lock()
    }

    fn unlock(&mut self) -> BoxFuture<'_, Result<(), MigrateError>> {
        self.conn.unlock()
    }

    fn apply<'e: 'm, 'm>(
        &'e mut self,
        migration: &'m Migration,
    ) -> BoxFuture<'m, Result<Duration, MigrateError>> {
        if !migration.description.ends_with(AUTOCOMMIT_SUFFIX) {
            tracing::info!(
                cluster = self.name,
                version = migration.version,
                description = %migration.description,
                "running migration inside of a transaction"
            );
            return self.conn.apply(migration);
        }

        // IMPORTANT: Copied *almost* verbatim from https://docs.rs/crate/sqlx-postgres/0.7.1/source/src/migrate.rs
        // except the migration is not run inside of a transaction
        Box::pin(async move {
            let start = Instant::now();

            tracing::info!(
                cluster = self.name,
                version = migration.version,
                description = %migration.description,
                "running migration outside of a transaction"
            );

            // language=SQL
            let _ = query(
                            r#"
                INSERT INTO _sqlx_migrations ( version, description, success, checksum, execution_time )
                VALUES ( $1, $2, FALSE, $3, -1 )
                            "#,
                        )
                        .bind(migration.version)
                        .bind(&*migration.description)
                        .bind(&*migration.checksum)
                        .execute(&mut self.conn)
                        .await?;

            for stmt in split_trusted_sql_statements(&migration.sql) {
                let _ = self.conn.execute(stmt.as_str()).await?;
            }

            // Update `elapsed_time`.
            // NOTE: The process may disconnect/die at this point, so the elapsed time value might be lost. We accept
            //       this small risk since this value is not super important.

            let elapsed = start.elapsed();

            // language=SQL
            let _ = query(
                r#"
    UPDATE _sqlx_migrations
    SET execution_time = $1, success = TRUE
    WHERE version = $2
                "#,
            )
            .bind(elapsed.as_nanos() as i64)
            .bind(migration.version)
            .execute(&mut self.conn)
            .await?;

            Ok(elapsed)
        })
    }

    fn revert<'e: 'm, 'm>(
        &'e mut self,
        migration: &'m Migration,
    ) -> BoxFuture<'m, Result<Duration, MigrateError>> {
        if !migration.description.ends_with(AUTOCOMMIT_SUFFIX) {
            tracing::info!(
                cluster = self.name,
                version = migration.version,
                description = %migration.description,
                "reverting migration inside of a transaction"
            );
            return self.conn.revert(migration);
        }

        // IMPORTANT: Copied *almost* verbatim from https://docs.rs/crate/sqlx-postgres/0.7.1/source/src/migrate.rs
        // except the revert is not run inside of a transaction
        Box::pin(async move {
            let start = Instant::now();

            tracing::info!(
                cluster = self.name,
                version = migration.version,
                description = %migration.description,
                "reverting migration outside of a transaction"
            );

            for stmt in split_trusted_sql_statements(&migration.sql) {
                let _ = self.conn.execute(stmt.as_str()).await?;
            }

            // language=SQL
            let _ = query(r#"DELETE FROM _sqlx_migrations WHERE version = $1"#)
                .bind(migration.version)
                .execute(&mut self.conn)
                .await?;

            let elapsed = start.elapsed();

            Ok(elapsed)
        })
    }
}
