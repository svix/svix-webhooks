// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::db::models::message;
use crate::error::Result;
use chrono::{Duration, Utc};
use sea_orm::{
    entity::prelude::*, sea_query::Expr, DatabaseConnection, DbErr, EntityTrait, UpdateResult,
};
use tokio::time::sleep;

/// Nullifies the payload column for expired messages
async fn clean_expired_messages(
    pool: &DatabaseConnection,
) -> std::result::Result<UpdateResult, DbErr> {
    // this logic exists to gracefully migrate existing message attempt responses
    message::Entity::update_many()
        .col_expr(
            message::Column::Expiration,
            Expr::value(Utc::now() + Duration::days(90)),
        )
        .filter(message::Column::Expiration.is_null())
        .filter(message::Column::Payload.is_not_null())
        .exec(pool)
        .await
        .expect("Update of existing messageattempt NULL timestamps failed");

    message::Entity::update_many()
        .col_expr(message::Column::Payload, Expr::value(Value::Json(None)))
        .filter(message::Column::Expiration.lte(Utc::now()))
        .filter(message::Column::Payload.is_not_null())
        .exec(pool)
        .await
}

/// Runs every 5 minutes
pub async fn expired_message_cleaner_loop(pool: &DatabaseConnection) -> Result<()> {
    loop {
        sleep(std::time::Duration::from_secs(5 * 60)).await;
        let pool = pool.clone();
        if let Err(err) = clean_expired_messages(&pool).await {
            tracing::error!("{}", err)
        }
    }
}
