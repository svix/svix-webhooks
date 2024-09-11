// SPDX-FileCopyrightText: © 2024 Svix Authors
// SPDX-License-Identifier: MIT

use reqwest::StatusCode;

use crate::utils::start_svix_server;

#[tokio::test]
async fn ping_with_trailing_slash() {
    let (client, _jh) = start_svix_server().await;

    client
        .get_without_response("api/v1/health/ping/", StatusCode::NO_CONTENT)
        .await
        .unwrap();
}

#[tokio::test]
async fn ping_without_trailing_slash() {
    let (client, _jh) = start_svix_server().await;

    client
        .get_without_response("api/v1/health/ping", StatusCode::NO_CONTENT)
        .await
        .unwrap();
}
