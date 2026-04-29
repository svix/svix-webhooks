// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT
use futures::future::join_all;
use sqlx::PgPool;
use svix_server::db::background_migrations::{
    BackgroundMigration, ensure_table, try_apply, try_revert,
};

use crate::utils::get_default_test_config;

async fn test_pool() -> PgPool {
    test_pool_with_conns(5).await
}

async fn test_pool_with_conns(num_conns: u32) -> PgPool {
    let cfg = get_default_test_config();
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(num_conns)
        .connect(&cfg.db_dsn)
        .await
        .unwrap()
}

async fn cleanup(pool: &PgPool, id: &str) {
    sqlx::query("DELETE FROM _svix_background_migrations WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_fresh_apply_succeeds() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    let migration = BackgroundMigration {
        id: "test_fresh_apply",
        apply_sql: &["SELECT 1"],
        cleanup_sql: None,
        revert_sql: None,
    };

    let done = try_apply(&pool, &migration).await.unwrap();
    assert!(done);

    let success: bool =
        sqlx::query_scalar("SELECT success FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(success);

    cleanup(&pool, migration.id).await;
}

#[tokio::test]
async fn test_apply_is_idempotent() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    let migration = BackgroundMigration {
        id: "test_idempotent",
        apply_sql: &["SELECT 1"],
        cleanup_sql: None,
        revert_sql: None,
    };

    try_apply(&pool, &migration).await.unwrap();
    let done = try_apply(&pool, &migration).await.unwrap();
    assert!(done);

    let attempts: i32 =
        sqlx::query_scalar("SELECT attempts FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(attempts, 0);

    cleanup(&pool, migration.id).await;
}

#[tokio::test]
async fn test_cleanup_runs_on_retry() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    sqlx::query("CREATE TABLE IF NOT EXISTS _test_bgmig_cleanup (id TEXT PRIMARY KEY)")
        .execute(&pool)
        .await
        .unwrap();

    // Seed a failed row so apply_inner takes the retry branch
    sqlx::query(
        "INSERT INTO _svix_background_migrations (id, success, attempts) VALUES ($1, FALSE, 1)
         ON CONFLICT (id) DO UPDATE SET success = FALSE, attempts = 1",
    )
    .bind("test_cleanup_retry")
    .execute(&pool)
    .await
    .unwrap();

    let migration = BackgroundMigration {
        id: "test_cleanup_retry",
        apply_sql: &["SELECT 1"],
        cleanup_sql: Some(&[
            "INSERT INTO _test_bgmig_cleanup VALUES ('ran') ON CONFLICT DO NOTHING",
        ]),
        revert_sql: None,
    };

    try_apply(&pool, &migration).await.unwrap();

    let result: Option<String> =
        sqlx::query_scalar("SELECT id FROM _test_bgmig_cleanup WHERE id = 'ran'")
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(result.is_some(),);

    sqlx::query("DROP TABLE _test_bgmig_cleanup")
        .execute(&pool)
        .await
        .unwrap();
    cleanup(&pool, migration.id).await;
}

#[tokio::test]
async fn test_failed_sql_records_error() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    let migration = BackgroundMigration {
        id: "test_failed_sql",
        apply_sql: &["this is not valid sql"],
        cleanup_sql: None,
        revert_sql: None,
    };

    let result = try_apply(&pool, &migration).await;
    assert!(result.is_err());

    let last_error: Option<String> =
        sqlx::query_scalar("SELECT last_error FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(last_error.is_some());

    cleanup(&pool, migration.id).await;
}

#[tokio::test]
async fn test_failed_cleanup_records_error() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    // Seed a failed row so apply_inner takes the retry/cleanup branch
    sqlx::query(
        "INSERT INTO _svix_background_migrations (id, success, attempts) VALUES ($1, FALSE, 1)
         ON CONFLICT (id) DO UPDATE SET success = FALSE, attempts = 1",
    )
    .bind("test_failed_cleanup")
    .execute(&pool)
    .await
    .unwrap();

    let migration = BackgroundMigration {
        id: "test_failed_cleanup",
        apply_sql: &["SELECT 1"],
        cleanup_sql: Some(&["this is not valid sql"]),
        revert_sql: None,
    };

    let result = try_apply(&pool, &migration).await;
    assert!(result.is_err());

    let last_error: Option<String> =
        sqlx::query_scalar("SELECT last_error FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(last_error.is_some());

    cleanup(&pool, migration.id).await;
}

#[tokio::test]
async fn test_revert_succeeds() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    sqlx::query("CREATE TABLE IF NOT EXISTS _test_bgmig_revert (id TEXT PRIMARY KEY)")
        .execute(&pool)
        .await
        .unwrap();

    let migration = BackgroundMigration {
        id: "test_revert",
        apply_sql: &["INSERT INTO _test_bgmig_revert VALUES ('applied') ON CONFLICT DO NOTHING"],
        cleanup_sql: None,
        revert_sql: Some(&["DELETE FROM _test_bgmig_revert WHERE id = 'applied'"]),
    };

    try_apply(&pool, &migration).await.unwrap();

    let row: Option<String> =
        sqlx::query_scalar("SELECT id FROM _test_bgmig_revert WHERE id = 'applied'")
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(row.is_some());

    try_revert(&pool, &migration).await.unwrap();

    let row: Option<String> =
        sqlx::query_scalar("SELECT id FROM _test_bgmig_revert WHERE id = 'applied'")
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(row.is_none());

    let tracking: Option<bool> =
        sqlx::query_scalar("SELECT success FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(tracking.is_none());

    sqlx::query("DROP TABLE _test_bgmig_revert")
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_empty_revert_sql_deletes_row() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    let migration = BackgroundMigration {
        id: "test_revert_no_sql",
        apply_sql: &["SELECT 1"],
        cleanup_sql: None,
        revert_sql: None,
    };

    try_apply(&pool, &migration).await.unwrap();

    let tracking_before: Option<bool> =
        sqlx::query_scalar("SELECT success FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(tracking_before.is_some());

    let reverted = try_revert(&pool, &migration).await.unwrap();
    assert!(reverted);

    let tracking_after: Option<bool> =
        sqlx::query_scalar("SELECT success FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(tracking_after.is_none());
}

#[tokio::test]
async fn test_revert_unapplied_migration_is_noop() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    let migration = BackgroundMigration {
        id: "test_revert_unapplied",
        apply_sql: &["SELECT 1"],
        cleanup_sql: None,
        revert_sql: Some(&["SELECT 1"]),
    };

    let reverted = try_revert(&pool, &migration).await.unwrap();
    assert!(!reverted);
}

#[tokio::test]
async fn test_concurrent_apply() {
    let pool = test_pool_with_conns(20).await;
    ensure_table(&pool).await.unwrap();

    sqlx::query("CREATE TABLE IF NOT EXISTS _test_bgmig_concurrent (id TEXT PRIMARY KEY)")
        .execute(&pool)
        .await
        .unwrap();

    let migration = BackgroundMigration {
        id: "test_concurrent",
        apply_sql: &["INSERT INTO _test_bgmig_concurrent VALUES (gen_random_uuid()::text)"],
        cleanup_sql: Some(&["DELETE FROM _test_bgmig_concurrent"]),
        revert_sql: None,
    };

    let results = join_all((0..8).map(|_| try_apply(&pool, &migration))).await;

    for result in results {
        assert!(result.is_ok());
    }

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM _test_bgmig_concurrent")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);

    let success: bool =
        sqlx::query_scalar("SELECT success FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(success);

    sqlx::query("DROP TABLE _test_bgmig_concurrent")
        .execute(&pool)
        .await
        .unwrap();
    cleanup(&pool, migration.id).await;
}

#[tokio::test]
async fn test_revert_failed_migration_is_noop() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    sqlx::query(
        "INSERT INTO _svix_background_migrations (id, success, attempts) VALUES ($1, FALSE, 1)
         ON CONFLICT (id) DO UPDATE SET success = FALSE, attempts = 1",
    )
    .bind("test_revert_failed")
    .execute(&pool)
    .await
    .unwrap();

    let migration = BackgroundMigration {
        id: "test_revert_failed",
        apply_sql: &["SELECT 1"],
        cleanup_sql: None,
        revert_sql: Some(&["SELECT 1"]),
    };

    let reverted = try_revert(&pool, &migration).await.unwrap();
    assert!(!reverted);

    cleanup(&pool, migration.id).await;
}
