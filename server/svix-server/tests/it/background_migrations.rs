// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT
use futures::future::join_all;
use sqlx::PgPool;
use svix_server::db::background_migrations::{
    BackgroundMigration, ensure_table, run_migrations, run_with_pool, try_revert,
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

    cleanup(&pool, migration.id).await;
    run_with_pool(&pool, &[migration]).await;

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

    cleanup(&pool, migration.id).await;
    run_with_pool(&pool, &[migration]).await;
    run_with_pool(&pool, &[migration]).await;

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

    sqlx::query("DROP TABLE IF EXISTS _test_bgmig_cleanup")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("CREATE TABLE _test_bgmig_cleanup (id TEXT PRIMARY KEY)")
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

    run_with_pool(&pool, &[migration]).await;

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

    cleanup(&pool, migration.id).await;
    run_migrations(&pool, &[migration]).await.unwrap_err();

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

    run_migrations(&pool, &[migration]).await.unwrap_err();

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

    sqlx::query("DROP TABLE IF EXISTS _test_bgmig_revert")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("CREATE TABLE _test_bgmig_revert (id TEXT PRIMARY KEY)")
        .execute(&pool)
        .await
        .unwrap();

    let migration = BackgroundMigration {
        id: "test_revert",
        apply_sql: &["INSERT INTO _test_bgmig_revert VALUES ('applied') ON CONFLICT DO NOTHING"],
        cleanup_sql: None,
        revert_sql: Some(&["DELETE FROM _test_bgmig_revert WHERE id = 'applied'"]),
    };

    cleanup(&pool, migration.id).await;
    run_with_pool(&pool, &[migration]).await;

    let row: Option<String> =
        sqlx::query_scalar("SELECT id FROM _test_bgmig_revert WHERE id = 'applied'")
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(row.is_some());

    try_revert(&pool, &[migration], migration.id).await.unwrap();

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

    cleanup(&pool, migration.id).await;
    run_with_pool(&pool, &[migration]).await;

    let tracking_before: Option<bool> =
        sqlx::query_scalar("SELECT success FROM _svix_background_migrations WHERE id = $1")
            .bind(migration.id)
            .fetch_optional(&pool)
            .await
            .unwrap();
    assert!(tracking_before.is_some());

    let reverted = try_revert(&pool, &[migration], migration.id).await.unwrap();
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
async fn test_revert_unapplied_migration_errors() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    let migration = BackgroundMigration {
        id: "test_revert_unapplied",
        apply_sql: &["SELECT 1"],
        cleanup_sql: None,
        revert_sql: Some(&["SELECT 1"]),
    };

    let result = try_revert(&pool, &[migration], migration.id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_concurrent_apply() {
    let pool = test_pool_with_conns(20).await;
    ensure_table(&pool).await.unwrap();

    // Drop and recreate to clear any state left by a previous failed test run.
    sqlx::query("DROP TABLE IF EXISTS _test_bgmig_concurrent")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("CREATE TABLE _test_bgmig_concurrent (id TEXT PRIMARY KEY)")
        .execute(&pool)
        .await
        .unwrap();

    let migration = BackgroundMigration {
        id: "test_concurrent",
        apply_sql: &["INSERT INTO _test_bgmig_concurrent VALUES (gen_random_uuid()::text)"],
        cleanup_sql: Some(&["DELETE FROM _test_bgmig_concurrent"]),
        revert_sql: None,
    };
    cleanup(&pool, migration.id).await;

    // Concurrent callers race on the tracking-table insert; only the one that
    // wins should execute the migration SQL.
    let migrations = [migration];
    join_all((0..8).map(|_| run_with_pool(&pool, &migrations))).await;

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
async fn test_revert_failed_migration_errors() {
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

    let result = try_revert(&pool, &[migration], migration.id).await;
    assert!(result.is_err());

    cleanup(&pool, migration.id).await;
}

#[tokio::test]
async fn test_failure_stops_subsequent_migrations() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    let migrations = [
        BackgroundMigration {
            id: "test_stop_on_fail_1",
            apply_sql: &["SELECT 1"],
            cleanup_sql: None,
            revert_sql: None,
        },
        BackgroundMigration {
            id: "test_stop_on_fail_2",
            apply_sql: &["this is not valid sql"],
            cleanup_sql: None,
            revert_sql: None,
        },
        BackgroundMigration {
            id: "test_stop_on_fail_3",
            apply_sql: &["SELECT 1"],
            cleanup_sql: None,
            revert_sql: None,
        },
        BackgroundMigration {
            id: "test_stop_on_fail_4",
            apply_sql: &["SELECT 1"],
            cleanup_sql: None,
            revert_sql: None,
        },
        BackgroundMigration {
            id: "test_stop_on_fail_5",
            apply_sql: &["SELECT 1"],
            cleanup_sql: None,
            revert_sql: None,
        },
    ];

    for m in &migrations {
        cleanup(&pool, m.id).await;
    }
    run_migrations(&pool, &migrations).await.unwrap_err();

    let ran: Vec<String> = sqlx::query_scalar(
        "SELECT id FROM _svix_background_migrations WHERE id LIKE 'test_stop_on_fail_%'",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert!(ran.contains(&"test_stop_on_fail_1".to_string()));
    assert!(ran.contains(&"test_stop_on_fail_2".to_string()));
    assert!(!ran.contains(&"test_stop_on_fail_3".to_string()));
    assert!(!ran.contains(&"test_stop_on_fail_4".to_string()));
    assert!(!ran.contains(&"test_stop_on_fail_5".to_string()));

    for m in &migrations {
        cleanup(&pool, m.id).await;
    }
}

#[tokio::test]
async fn test_revert_non_latest_errors() {
    let pool = test_pool().await;
    ensure_table(&pool).await.unwrap();

    let m1 = BackgroundMigration {
        id: "test_revert_non_latest_m1",
        apply_sql: &["SELECT 1"],
        cleanup_sql: None,
        revert_sql: Some(&["SELECT 1"]),
    };
    let m2 = BackgroundMigration {
        id: "test_revert_non_latest_m2",
        apply_sql: &["SELECT 1"],
        cleanup_sql: None,
        revert_sql: Some(&["SELECT 1"]),
    };
    let migrations = [m1, m2];

    cleanup(&pool, m1.id).await;
    cleanup(&pool, m2.id).await;
    run_with_pool(&pool, &migrations).await;

    let result = try_revert(&pool, &migrations, migrations[0].id).await;
    assert!(result.is_err());

    cleanup(&pool, migrations[0].id).await;
    cleanup(&pool, migrations[1].id).await;
}
