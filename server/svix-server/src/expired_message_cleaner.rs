// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::{
    sync::atomic::Ordering,
    time::{Duration, Instant},
};

use sea_orm::{
    ConnectionTrait, DatabaseConnection, DbErr, ExecResult, QueryResult, Statement,
    TransactionTrait, UpdateResult,
};

use crate::error::{Error, Result};

type DbResult<T> = std::result::Result<T, DbErr>;

async fn exec_without_timeout(pool: &DatabaseConnection, stmt: Statement) -> DbResult<ExecResult> {
    let increase_timeout = Statement::from_string(
        pool.get_database_backend(),
        "SET LOCAL statement_timeout=0;",
    );
    let tx = pool.begin().await?;
    let _ = tx.execute(increase_timeout).await?;
    let res = tx.execute(stmt).await?;
    tx.commit().await?;
    Ok(res)
}
async fn query_one_without_timeout(
    pool: &DatabaseConnection,
    stmt: Statement,
) -> DbResult<Option<QueryResult>> {
    let increase_timeout = Statement::from_string(
        pool.get_database_backend(),
        "SET LOCAL statement_timeout=0;",
    );
    let tx = pool.begin().await?;
    let _ = tx.execute(increase_timeout).await?;
    let res = tx.query_one(stmt).await?;
    tx.commit().await?;
    Ok(res)
}

/// Nullifies the payload column for expired messages,
/// `limit` sets how many rows to update at a time.
pub async fn clean_expired_messages(
    pool: &DatabaseConnection,
    limit: u32,
    enable_legacy_message_cleaner: bool,
) -> DbResult<UpdateResult> {
    // See the docs for [`has_message_payloads_pending_expiry`] for background on the legacy cleaner.
    let legacy_row_count = if enable_legacy_message_cleaner {
        let legacy_res = {
            let legacy_stmt = Statement::from_sql_and_values(
                pool.get_database_backend(),
                r#"
        UPDATE message SET payload = NULL WHERE id IN (
            SELECT id FROM message
            WHERE
                expiration <= now()
                AND payload IS NOT NULL
            LIMIT $1
            FOR UPDATE SKIP LOCKED
        )
    "#,
                [limit.into()],
            );

            exec_without_timeout(pool, legacy_stmt).await?
        };
        legacy_res.rows_affected()
    } else {
        0
    };

    let stmt = Statement::from_sql_and_values(
        pool.get_database_backend(),
        r#"
        DELETE FROM messagecontent WHERE id = any(
            array(
                SELECT id FROM messagecontent
                WHERE 
                    expiration <= now()
                LIMIT $1
                FOR UPDATE SKIP LOCKED
            )
        )
    "#,
        [limit.into()],
    );
    let res = pool.execute(stmt).await?;

    Ok(UpdateResult {
        rows_affected: legacy_row_count + res.rows_affected(),
    })
}

/// Checks to see if the message table has any non-null payloads requiring expiry.
///
/// ## Background
///
/// Initially payloads were modeled as a field in `message`, but later migrated to a separate
/// table (`messagecontent`). In cases where there are no longer any payloads to expire in `message` we
/// can avoid the expense of running the cleaner on the `message` table since all new messages should now be using
/// `messagecontent`.
async fn has_message_payloads_pending_expiry(pool: &DatabaseConnection) -> Result<bool> {
    query_one_without_timeout(
        pool,
        Statement::from_string(
            pool.get_database_backend(),
            r#"SELECT EXISTS (SELECT 1 FROM message WHERE payload IS NOT NULL LIMIT 1)"#,
        ),
    )
    .await?
    .ok_or_else(|| Error::generic("failed to check for message payloads"))?
    .try_get_by_index(0)
    .map_err(|e| Error::generic(format!("failed to check for message payloads: {e}")))
}

/// Polls the database for expired messages to nullify payloads for.
///
/// Uses a variable polling schedule, based on affected row counts each iteration of the loop.
pub async fn expired_message_cleaner_loop(pool: &DatabaseConnection) -> Result<()> {
    let message_table_needs_cleaning = has_message_payloads_pending_expiry(pool).await?;
    if !message_table_needs_cleaning {
        tracing::info!("No payloads pending expiry found in `message` table. Skipping the cleaner for this table.");
    }

    // When fewer rows than the batch size have been updated, take a nap for this long.
    const IDLE: Duration = Duration::from_secs(60 * 60 * 12);
    const ON_ERROR: Duration = Duration::from_secs(10);
    const BATCH_SIZE: u32 = 5_000;
    let mut sleep_time = None;
    loop {
        if let Some(duration) = sleep_time {
            let sleep_start = Instant::now();
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            interval.tick().await;
            // Doing a plain sleep() was fine when the polling frequency was mere seconds, but since we're doing wider
            // periods now (hours, not seconds), we need to be a little more careful about not preventing the process
            // from shutting down.
            // Using `interval()` so we can track how long we've been sleeping for, while still checking for the
            // shutdown signal.
            'inner: loop {
                if crate::SHUTTING_DOWN.load(Ordering::SeqCst) {
                    return Ok(());
                }
                interval.tick().await;
                if sleep_start.elapsed() > duration {
                    break 'inner;
                }
            }
        }

        let start = Instant::now();
        match clean_expired_messages(pool, BATCH_SIZE, message_table_needs_cleaning).await {
            Err(err) => {
                tracing::error!("{}", err);
                sleep_time = Some(ON_ERROR);
            }
            Ok(UpdateResult { rows_affected }) => {
                if rows_affected > 0 {
                    tracing::debug!(elapsed =? start.elapsed(), "expired {} payloads", rows_affected);
                }

                sleep_time = if rows_affected < (BATCH_SIZE as _) {
                    Some(IDLE)
                } else {
                    // When we see full batches, don't sleep at all.
                    None
                };
            }
        }

        if crate::SHUTTING_DOWN.load(Ordering::SeqCst) {
            break;
        }
    }

    Ok(())
}
