// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use chrono::{DateTime, Utc};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, DbErr, DeleteResult, EntityTrait,
    ExecResult, QueryFilter, SqlxPostgresConnector, Statement, TransactionTrait,
};
use sqlx::postgres::PgPoolOptions;

use crate::{
    cfg::Configuration,
    core::types::{BaseId, MessageId, OrganizationId},
};

pub mod models;
use models::{application, endpoint, eventtype, message, messageattempt};

static MIGRATIONS: sqlx::migrate::Migrator = sqlx::migrate!();

async fn connect(dsn: &str, max_pool_size: u16) -> sqlx::Pool<sqlx::Postgres> {
    if DbBackend::Postgres.is_prefix_of(dsn) {
        PgPoolOptions::new()
            .max_connections(max_pool_size.into())
            .connect(dsn)
            .await
            .expect("Error connecting to Postgres")
    } else {
        panic!("db_dsn format not recognized. {dsn}")
    }
}

pub async fn init_db(cfg: &Configuration) -> DatabaseConnection {
    SqlxPostgresConnector::from_sqlx_postgres_pool(connect(&cfg.db_dsn, cfg.db_pool_max_size).await)
}

pub async fn run_migrations(cfg: &Configuration) {
    let db = connect(&cfg.db_dsn, cfg.db_pool_max_size).await;
    MIGRATIONS.run(&db).await.unwrap();
}

async fn exec_without_timeout(
    db: &DatabaseConnection,
    stmt: Statement,
) -> Result<ExecResult, DbErr> {
    let tx = db.begin().await?;
    tx.execute(Statement::from_string(
        db.get_database_backend(),
        "SET LOCAL statement_timeout=0",
    ))
    .await?;
    let res = tx.execute(stmt).await?;
    tx.commit().await?;
    Ok(res)
}

/// Prune messages (and their attempts) older than `older_than` in batches.
///
/// `messagecontent` is intentionally skipped: the background cleaner expires it at 90 days,
/// so anything old enough to prune here will already be gone.
pub async fn prune_messages(
    cfg: &Configuration,
    older_than: DateTime<Utc>,
    batch_size: u64,
) -> anyhow::Result<()> {
    let db = init_db(cfg).await;
    let cutoff_id = MessageId::start_id(older_than);
    let cutoff_str = cutoff_id.to_string();

    // Step 1: messageattempt
    let mut total_attempts: u64 = 0;
    loop {
        let stmt = Statement::from_sql_and_values(
            db.get_database_backend(),
            r#"DELETE FROM messageattempt WHERE id IN (
                SELECT id FROM messageattempt WHERE msg_id < $1 ORDER BY msg_id LIMIT $2
            )"#,
            [cutoff_str.clone().into(), (batch_size as i64).into()],
        );
        let rows = exec_without_timeout(&db, stmt).await?.rows_affected();
        total_attempts += rows;
        tracing::info!(rows, "Pruned batch of messageattempt row(s)");
        if rows < batch_size {
            break;
        }
    }

    // Step 2: message
    let mut total_messages: u64 = 0;
    loop {
        let stmt = Statement::from_sql_and_values(
            db.get_database_backend(),
            r#"DELETE FROM message WHERE id IN (
                SELECT id FROM message WHERE id < $1 ORDER BY id LIMIT $2
            )"#,
            [cutoff_str.clone().into(), (batch_size as i64).into()],
        );
        let rows = exec_without_timeout(&db, stmt).await?.rows_affected();
        total_messages += rows;
        tracing::info!(rows, "Pruned batch of message row(s)");
        if rows < batch_size {
            break;
        }
    }

    eprintln!("Done. Pruned {total_attempts} messageattempt(s) and {total_messages} message(s).");

    Ok(())
}

/// Wipe an organization from existence in a way that ensures the operation can be tried again on
/// failure.
pub async fn wipe_org(cfg: &Configuration, org_id: OrganizationId) {
    let db = init_db(cfg).await;

    let applications: Vec<application::Model> = application::Entity::secure_find(org_id.clone())
        .all(&db)
        .await
        .unwrap_or_else(|_| panic!("Error fetching applications associated with org ID {org_id}"));

    for application in applications {
        let endpoints: Vec<endpoint::Model> = endpoint::Entity::secure_find(application.id.clone())
            .all(&db)
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "Error fetching endpoints associated with application ID {}",
                    application.id
                )
            });

        for endpoint in endpoints {
            // First [`messageattempt`]s, then [`messagedestination`]s
            let _: DeleteResult = messageattempt::Entity::delete_many()
                .filter(messageattempt::Column::EndpId.eq(endpoint.id.clone()))
                .exec(&db)
                .await
                .unwrap_or_else(|_| {
                    panic!(
                        "Error deleting messageattempts associated with endpoint ID {}",
                        endpoint.id
                    )
                });
        }

        // Then [`message`]s, then [`endpoint`]s
        let _: DeleteResult = message::Entity::delete_many()
            .filter(message::Column::AppId.eq(application.id.clone()))
            .exec(&db)
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "Error deleting messages associated with application ID {}",
                    application.id
                )
            });

        let _: DeleteResult = endpoint::Entity::delete_many()
            .filter(endpoint::Column::AppId.eq(application.id.clone()))
            .exec(&db)
            .await
            .unwrap_or_else(|_| {
                panic!(
                    "Error deleting endpoints associated with application ID {}",
                    application.id
                )
            });
    }

    // Then [`application`]s, then [`eventtype`]s
    let _: DeleteResult = application::Entity::delete_many()
        .filter(application::Column::OrgId.eq(org_id.clone()))
        .exec(&db)
        .await
        .unwrap_or_else(|_| panic!("Error deleting applications associated with org ID {org_id}"));

    let _: DeleteResult = eventtype::Entity::delete_many()
        .filter(eventtype::Column::OrgId.eq(org_id.clone()))
        .exec(&db)
        .await
        .unwrap_or_else(|_| panic!("Error deleting event types associated with org ID {org_id}"));
}
