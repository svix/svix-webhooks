// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use sqlx::{PgPool, Row as _};

use crate::cfg::Configuration;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("{0}")]
    Migration(String),
}

const GLOBAL_LOCK_ID: &str = "_svix_background_migrations";

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

#[derive(Copy, Clone)]
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

async fn advisory_lock_acquire(conn: &mut sqlx::PgConnection) -> Result<bool, sqlx::Error> {
    let key = lock_key(GLOBAL_LOCK_ID);
    sqlx::query_scalar("SELECT pg_try_advisory_lock($1)")
        .bind(key)
        .fetch_one(conn)
        .await
}

async fn advisory_lock_release(conn: &mut sqlx::PgConnection) {
    let key = lock_key(GLOBAL_LOCK_ID);
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

pub async fn run_migrations(
    pool: &PgPool,
    migrations: &[BackgroundMigration],
) -> Result<bool, sqlx::Error> {
    let mut conn = pool.acquire().await?;

    if !advisory_lock_acquire(&mut conn).await? {
        return Ok(false);
    }

    let mut apply_result = Ok(());
    for migration in migrations {
        if let Err(e) = apply(pool, migration).await {
            apply_result = Err(e);
            break;
        }
    }

    advisory_lock_release(&mut conn).await;
    apply_result?;
    Ok(true)
}

async fn apply(pool: &PgPool, migration: &BackgroundMigration) -> Result<(), sqlx::Error> {
    let existing =
        sqlx::query("SELECT success, attempts FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_optional(pool)
            .await?;

    match existing {
        Some(row) if row.get::<bool, _>("success") => return Ok(()),
        // Attempt to clean up previously failed migration
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

    if let Err(e) = apply_result {
        let _ = sqlx::query("UPDATE _svix_background_migrations SET last_error = $1 WHERE id = $2")
            .bind(e.to_string())
            .bind(migration.id)
            .execute(pool)
            .await;
        return Err(e);
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

    Ok(())
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
        match run_migrations(pool, migrations).await {
            Ok(true) => {
                tracing::debug!("Background migrations complete. Exiting.");
                return;
            }
            Ok(false) => {
                tracing::info!("Background migrations are already locked. Exiting.");
                return;
            }
            Err(e) => {
                tracing::error!(error = %e, "Background migration error");
            }
        }

        tokio::select! {
            _ = tokio::time::sleep(backoff) => {}
            _ = shutdown.cancelled() => return,
        }
        backoff = (backoff * 2).min(Duration::from_secs(600));
    }
}

pub async fn try_revert(
    pool: &PgPool,
    migrations: &[BackgroundMigration],
    id: &str,
) -> Result<bool, Error> {
    let mut conn = pool.acquire().await?;

    if !advisory_lock_acquire(&mut conn).await? {
        return Ok(false);
    }

    let applied: std::collections::HashSet<String> =
        sqlx::query_scalar("SELECT id FROM _svix_background_migrations WHERE success = TRUE")
            .fetch_all(pool)
            .await?
            .into_iter()
            .collect();

    let Some(last_applied) = migrations.iter().rev().find(|m| applied.contains(m.id)) else {
        return Err(Error::Migration(format!(
            "migration '{id}' has not been applied"
        )));
    };
    if last_applied.id != id {
        return Err(Error::Migration(format!(
            "migration '{id}' is not the last applied migration and cannot be reverted out of order"
        )));
    }

    if last_applied.revert_sql.is_none() {
        tracing::warn!(id, "No revert SQL for migration");
    }

    let result = revert_inner(pool, last_applied, last_applied.revert_sql).await;
    advisory_lock_release(&mut conn).await;
    result
}

// If there's no actual revert script, only the migrations table row
// will be deleted.
async fn revert_inner(
    pool: &PgPool,
    migration: &BackgroundMigration,
    statements: Option<&[&str]>,
) -> Result<bool, Error> {
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
                return Err(e.into());
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
