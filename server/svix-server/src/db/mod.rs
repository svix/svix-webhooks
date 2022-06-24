// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use sea_orm::{DatabaseConnection, DbBackend, SqlxPostgresConnector};
use sqlx::postgres::PgPoolOptions;

use crate::cfg::Configuration;

pub mod models;

static MIGRATIONS: sqlx::migrate::Migrator = sqlx::migrate!();

#[tracing::instrument(level = "trace")]
async fn connect(cfg: &Configuration) -> sqlx::Pool<sqlx::Postgres> {
    tracing::debug!("DB: Initializing pool");
    if DbBackend::Postgres.is_prefix_of(&cfg.db_dsn) {
        PgPoolOptions::new()
            .max_connections(cfg.db_pool_max_size.into())
            .connect(&cfg.db_dsn)
            .await
            .expect("Error connectiong to Postgres")
    } else {
        panic!("db_dsn format not recognized. {}", &cfg.db_dsn)
    }
}

#[tracing::instrument(level = "trace")]
pub async fn init_db(cfg: &Configuration) -> DatabaseConnection {
    SqlxPostgresConnector::from_sqlx_postgres_pool(connect(cfg).await)
}

#[tracing::instrument(level = "trace")]
pub async fn run_migrations(cfg: &Configuration) {
    let db = connect(cfg).await;
    MIGRATIONS.run(&db).await.unwrap();
}
