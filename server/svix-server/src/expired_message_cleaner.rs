// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::sync::atomic::Ordering;

use crate::db::models::message;
use crate::error::Result;
use chrono::Utc;
use sea_orm::{
    entity::prelude::*, sea_query::Expr, DatabaseConnection, DbErr, EntityTrait, UpdateResult,
};
use tokio::time::sleep;

/// Nullifies the payload column for expired messages
pub async fn clean_expired_messages(
    pool: &DatabaseConnection,
) -> std::result::Result<UpdateResult, DbErr> {
    message::Entity::update_many()
        .col_expr(message::Column::Payload, Expr::value(Value::Json(None)))
        .filter(message::Column::Expiration.lte(Utc::now()))
        .filter(message::Column::Payload.is_not_null())
        .exec(pool)
        .await
}

/// Runs every 10 seconds
pub async fn expired_message_cleaner_loop(pool: &DatabaseConnection) -> Result<()> {
    loop {
        sleep(std::time::Duration::from_secs(10)).await;
        let pool = pool.clone();
        if let Err(err) = clean_expired_messages(&pool).await {
            tracing::error!("{}", err)
        }

        if crate::SHUTTING_DOWN.load(Ordering::SeqCst) {
            break;
        }
    }

    Ok(())
}
