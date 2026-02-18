// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use sea_orm::{
    ColumnTrait, DatabaseConnection, DbBackend, DeleteResult, EntityTrait, QueryFilter,
    SqlxPostgresConnector, sqlx::migrate::Migrate,
};
use sqlx::postgres::PgPoolOptions;

use crate::{cfg::Configuration, core::types::OrganizationId, db::migrator::TxnOptionalMigrator};

pub mod migrator;
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
    let (version, description) = MIGRATIONS
        .migrations
        .last()
        .map(|m| (m.version, &m.description))
        .expect("Where the migrations at?");

    tracing::info!("Migrations: starting (most recent in source: {version} '{description}')");

    // sqlx::Pool only runs migrations in a transaction, which prevents us from doing things
    // like CREATE INDEX CONCURRENTLY . . .
    // So we wrap our pools in a special migrator type that avoids transactions.
    let mut migrator = TxnOptionalMigrator::new("db", &db).await;

    tracing::info!("Migrating database");
    migrator
        .ensure_migrations_table()
        .await
        .expect("Failed to ensure db");
    MIGRATIONS
        .run_direct(&mut migrator)
        .await
        .expect("Unable to migrate db");

    tracing::info!("Migrations: Done!");
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
