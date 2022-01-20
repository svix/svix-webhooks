// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use sea_orm::{DatabaseConnection, DbBackend, SqlxPostgresConnector};
use sqlx::postgres::PgPoolOptions;

use crate::cfg::Configuration;

pub mod models;

#[cfg(not(debug_assertions))]
static MIGRATIONS: sqlx::migrate::Migrator = sqlx::migrate!();

pub async fn init_db(cfg: &Configuration) -> DatabaseConnection {
    tracing::debug!("DB: Initializing pool");
    if DbBackend::Postgres.is_prefix_of(&cfg.db_dsn) {
        let sqlx_pool = PgPoolOptions::new().connect(&cfg.db_dsn).await.unwrap();

        #[cfg(not(debug_assertions))]
        MIGRATIONS.run(&sqlx_pool).await.unwrap();

        SqlxPostgresConnector::from_sqlx_postgres_pool(sqlx_pool)
    } else {
        panic!("db_dsn format not recognized. {}", &cfg.db_dsn)
    }
}
