// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use sqlx::{PgPool, Row as _};

use crate::cfg::Configuration;

// Background migrations go here:
pub static MIGRATIONS: &[BackgroundMigration] = &[BackgroundMigration {
    id: "2026_04_1777303796_messageattempt_per_endp_no_status",
    apply_sql: &[r#"
        CREATE INDEX CONCURRENTLY IF NOT EXISTS messageattempt_per_endp_no_status
        ON messageattempt USING btree (endp_id, id DESC)
        INCLUDE (status, response_status_code)
    "#],
    cleanup_sql: None,
    revert_sql: Some(&["DROP INDEX CONCURRENTLY IF EXISTS messageattempt_per_endp_no_status"]),
}];

pub struct BackgroundMigration {
    /// Unique id for the migration.
    pub id: &'static str,
    /// Sql statements to apply the migration.
    pub apply_sql: &'static [&'static str],
    /// Optional Sql statements that should be run before re-applying if the migration fails.
    pub cleanup_sql: Option<&'static [&'static str]>,
    /// Optional Sql statements to run for reverting the migration.
    pub revert_sql: Option<&'static [&'static str]>,
}

async fn advisory_lock_acquire(
    conn: &mut sqlx::PgConnection,
    key: i64,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar("SELECT pg_try_advisory_lock($1)")
        .bind(key)
        .fetch_one(conn)
        .await
}

async fn advisory_lock_release(conn: &mut sqlx::PgConnection, key: i64) {
    if let Err(e) = sqlx::query("SELECT pg_advisory_unlock($1)")
        .bind(key)
        .execute(conn)
        .await
    {
        tracing::warn!(error = %e, "Failed to release advisory lock");
    }
}

// Modeled after sqlx implementation
fn lock_key(id: &str) -> i64 {
    const CRC_IEEE: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
    0x7a9f2c51 * (CRC_IEEE.checksum(id.as_bytes()) as i64)
}

pub async fn ensure_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS _svix_background_migrations (
            id          TEXT        PRIMARY KEY,
            started_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
            finished_at TIMESTAMPTZ,
            success     BOOLEAN     NOT NULL DEFAULT FALSE,
            attempts    INTEGER     NOT NULL DEFAULT 0,
            last_error  TEXT
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn try_apply(
    pool: &PgPool,
    migration: &BackgroundMigration,
) -> Result<bool, sqlx::Error> {
    let key = lock_key(migration.id);

    let mut conn = pool.acquire().await?;

    if !advisory_lock_acquire(&mut conn, key).await? {
        let done: Option<bool> =
            sqlx::query_scalar("SELECT success FROM _svix_background_migrations WHERE id = $1")
                .bind(migration.id)
                .fetch_optional(pool)
                .await?;
        return Ok(done.unwrap_or(false));
    }

    let result = apply_inner(pool, migration).await;
    advisory_lock_release(&mut conn, key).await;
    result
}

async fn apply_inner(pool: &PgPool, migration: &BackgroundMigration) -> Result<bool, sqlx::Error> {
    let existing =
        sqlx::query("SELECT success, attempts FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_optional(pool)
            .await?;

    match existing {
        Some(row) if row.get::<bool, _>("success") => return Ok(true),
        Some(row) => {
            let attempts: i32 = row.get("attempts");
            tracing::warn!(
                id = migration.id,
                attempts,
                "Background migration previously failed"
            );
            if let Some(stmts) = migration.cleanup_sql {
                tracing::info!(id = migration.id, "Running cleanup before retry");
                for stmt in stmts {
                    if let Err(e) = sqlx::query(stmt).execute(pool).await {
                        let _ = sqlx::query(
                            "UPDATE _svix_background_migrations SET last_error = $1 WHERE id = $2",
                        )
                        .bind(e.to_string())
                        .bind(migration.id)
                        .execute(pool)
                        .await;
                        return Err(e);
                    }
                }
                tracing::info!(id = migration.id, "Cleanup completed");
            }
            sqlx::query(
                "UPDATE _svix_background_migrations
                 SET attempts = attempts + 1, last_error = NULL
                 WHERE id = $1",
            )
            .bind(migration.id)
            .execute(pool)
            .await?;
        }
        None => {
            sqlx::query("INSERT INTO _svix_background_migrations (id) VALUES ($1)")
                .bind(migration.id)
                .execute(pool)
                .await?;
        }
    }

    tracing::info!(id = migration.id, "Starting background migration");
    let started = std::time::Instant::now();

    let apply_result: Result<(), sqlx::Error> = async {
        for stmt in migration.apply_sql {
            sqlx::query(stmt).execute(pool).await?;
        }
        Ok(())
    }
    .await;

    if let Err(ref e) = apply_result {
        let _ = sqlx::query("UPDATE _svix_background_migrations SET last_error = $1 WHERE id = $2")
            .bind(e.to_string())
            .bind(migration.id)
            .execute(pool)
            .await;
        return apply_result.map(|_| false);
    }

    sqlx::query(
        "UPDATE _svix_background_migrations
         SET success = TRUE, finished_at = now()
         WHERE id = $1",
    )
    .bind(migration.id)
    .execute(pool)
    .await?;

    tracing::info!(
        id = migration.id,
        elapsed_secs = started.elapsed().as_secs_f64(),
        "Background migration completed"
    );
    Ok(true)
}

pub async fn run(cfg: &Configuration) {
    let pool = super::connect(&cfg.db_dsn, cfg.db_pool_max_size).await;
    run_with_pool(&pool, MIGRATIONS).await;
}

pub async fn run_with_pool(pool: &PgPool, migrations: &[BackgroundMigration]) {
    if let Err(e) = ensure_table(pool).await {
        tracing::error!(error = %e, "Failed to create _svix_background_migrations table");
        return;
    }

    let shutdown = crate::shutting_down_token();
    let mut backoff = Duration::from_secs(5);

    loop {
        let mut pending = false;

        for migration in migrations {
            match try_apply(pool, migration).await {
                Ok(true) => {}
                Ok(false) => {
                    pending = true;
                    break;
                }
                Err(e) => {
                    tracing::error!(
                        id = migration.id,
                        error = %e,
                        "Background migration failed"
                    );
                    pending = true;
                    break;
                }
            }
        }

        if !pending {
            tracing::debug!("Background migration worker exiting.");
            break;
        }

        tokio::select! {
            _ = tokio::time::sleep(backoff) => {}
            _ = shutdown.cancelled() => break,
        }
        backoff = (backoff * 2).min(Duration::from_secs(600));
    }
}

pub async fn try_revert(
    pool: &PgPool,
    migration: &BackgroundMigration,
) -> Result<bool, sqlx::Error> {
    if migration.revert_sql.is_none() {
        tracing::warn!(id = migration.id, "No revert SQl for migration");
    }

    let key = lock_key(migration.id);
    let mut conn = pool.acquire().await?;

    if !advisory_lock_acquire(&mut conn, key).await? {
        return Ok(false);
    }

    let result = revert_inner(pool, migration, migration.revert_sql).await;
    advisory_lock_release(&mut conn, key).await;
    result
}

// If there's no actual revert script, only the migrations table row
// will be deleted.
async fn revert_inner(
    pool: &PgPool,
    migration: &BackgroundMigration,
    statements: Option<&[&str]>,
) -> Result<bool, sqlx::Error> {
    let success: Option<bool> =
        sqlx::query_scalar("SELECT success FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_optional(pool)
            .await?;

    if success != Some(true) {
        return Ok(false);
    }

    tracing::info!(id = migration.id, "Reverting background migration");
    if let Some(statements) = statements {
        for stmt in statements {
            if let Err(e) = sqlx::query(stmt).execute(pool).await {
                let _ = sqlx::query(
                    "UPDATE _svix_background_migrations SET last_error = $1 WHERE id = $2",
                )
                .bind(e.to_string())
                .bind(migration.id)
                .execute(pool)
                .await;
                return Err(e);
            }
        }
    }

    sqlx::query("DELETE FROM _svix_background_migrations WHERE id = $1")
        .bind(migration.id)
        .execute(pool)
        .await?;

    tracing::info!(id = migration.id, "Background migration reverted");
    Ok(true)
}
