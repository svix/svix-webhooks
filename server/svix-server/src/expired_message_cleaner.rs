// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::sync::atomic::Ordering;

use crate::error::Result;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Statement, UpdateResult};
use std::time::Duration;
use tokio::time::sleep;

/// Nullifies the payload column for expired messages,
/// `limit` sets how many rows to update at a time.
pub async fn clean_expired_messages(
    pool: &DatabaseConnection,
    limit: u32,
) -> std::result::Result<UpdateResult, DbErr> {
    let stmt = Statement::from_sql_and_values(
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
    let res = pool.execute(stmt).await?;
    Ok(UpdateResult {
        rows_affected: res.rows_affected(),
    })
}

/// Polls the database for expired messages to nullify payloads for.
///
/// Uses a variable polling schedule, based on affected row counts each iteration of the loop.
pub async fn expired_message_cleaner_loop(pool: &DatabaseConnection) -> Result<()> {
    // When no rows have been updated, widen the interval.
    const IDLE: Duration = Duration::from_secs(10);
    // When the affected row count dips below this, switch to the `SLOWING` interval.
    const SLOWING_THRESHOLD: u64 = 1_000;
    const SLOWING: Duration = Duration::from_secs(3);
    const BATCH_SIZE: u32 = 5_000;
    let mut sleep_time = Some(IDLE);
    loop {
        if let Some(duration) = sleep_time {
            sleep(duration).await;
        }
        let pool = pool.clone();
        match clean_expired_messages(&pool, BATCH_SIZE).await {
            Err(err) => {
                tracing::error!("{}", err);
            }
            Ok(UpdateResult { rows_affected }) => {
                if rows_affected > 0 {
                    tracing::trace!("expired {} payloads", rows_affected);
                }

                sleep_time = match rows_affected {
                    0 => Some(IDLE),
                    count if count <= SLOWING_THRESHOLD => {
                        tracing::trace!("slowing down...");
                        Some(SLOWING)
                    }
                    // Any non-zero count above the slowing threshold gets no sleep.
                    _ => None,
                };
            }
        }

        if crate::SHUTTING_DOWN.load(Ordering::SeqCst) {
            break;
        }
    }

    Ok(())
}
