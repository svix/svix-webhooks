// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use reqwest::StatusCode;

mod utils;

use svix_server::v1::endpoints::health::{HealthReport, HealthStatus};
use utils::{get_default_test_config, start_svix_server, start_svix_server_with_cfg};

#[tokio::test]
async fn test_health() {
    let (client, _jh) = start_svix_server().await;

    let health_info: HealthReport = client.get("api/v1/health/", StatusCode::OK).await.unwrap();

    assert_eq!(health_info.queue, HealthStatus::new_ok());
    assert_eq!(health_info.database, HealthStatus::new_ok());
    assert_eq!(health_info.cache, HealthStatus::new_ok());
}

#[tokio::test]
async fn test_health_na() {
    let mut cfg = get_default_test_config();
    cfg.queue_health_check_enabled = false;
    cfg.cache_health_check_enabled = false;
    cfg.db_health_check_enabled = false;

    let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

    let health_info: HealthReport = client.get("api/v1/health/", StatusCode::OK).await.unwrap();

    assert_eq!(health_info.queue, HealthStatus::new_na());
    assert_eq!(health_info.database, HealthStatus::new_na());
    assert_eq!(health_info.cache, HealthStatus::new_na());
}
