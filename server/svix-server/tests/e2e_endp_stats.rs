// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use crate::utils::IgnoredResponse;
use chrono::prelude::*;
use reqwest::StatusCode;
use svix_server::v1::endpoints::stats::{AttemptStatisticsResponse, StatisticsPeriod};

mod utils;
use utils::{common_calls::create_test_app, start_svix_server};

fn dt_to_string(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[tokio::test]
async fn test_app_attempt_stats_defaults() {
    let (client, _jh) = start_svix_server().await;
    let app = create_test_app(&client, "app_attempt_stats")
        .await
        .unwrap()
        .id;

    let stats = client
        .get::<AttemptStatisticsResponse>(
            &format!("api/v1/stats/app/{app}/attempt/"),
            StatusCode::OK,
        )
        .await
        .unwrap();

    let now = Utc::now();
    let expected_end = Utc
        .with_ymd_and_hms(
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute() - (now.minute() % 5),
            0,
        )
        .unwrap();
    let expected_delta = chrono::Duration::hours(6);
    let expected_start = expected_end - expected_delta;
    let expected_bucket_count: usize = (expected_delta.num_minutes() / 5).try_into().unwrap();
    assert_eq!(stats.common_.start_date, expected_start);
    assert_eq!(stats.common_.end_date, expected_end);
    assert_eq!(stats.common_.period, StatisticsPeriod::FiveMinutes);
    assert!(stats.data.success_count.is_some());
    assert!(stats.data.failure_count.is_some());
    assert_eq!(
        stats.data.success_count.unwrap().len(),
        expected_bucket_count
    );
    assert_eq!(
        stats.data.failure_count.unwrap().len(),
        expected_bucket_count
    );
}

#[tokio::test]
async fn test_app_attempt_rounding() {
    let (client, _jh) = start_svix_server().await;

    let expected_delta = chrono::Duration::hours(12);
    let now = Utc::now();
    let provided_end = now;
    let provided_start = now - expected_delta;

    let app = create_test_app(&client, "app_attempt_stats")
        .await
        .unwrap()
        .id;

    let stats = client
        .get::<AttemptStatisticsResponse>(
            &format!(
                "api/v1/stats/app/{app}/attempt/?startDate={}&endDate={}",
                dt_to_string(provided_start),
                dt_to_string(provided_end)
            ),
            StatusCode::OK,
        )
        .await
        .unwrap();

    let expected_end = Utc
        .with_ymd_and_hms(
            provided_end.year(),
            provided_end.month(),
            provided_end.day(),
            provided_end.hour(),
            provided_end.minute() - (provided_end.minute() % 5),
            0,
        )
        .unwrap();
    let expected_start = Utc
        .with_ymd_and_hms(
            provided_start.year(),
            provided_start.month(),
            provided_start.day(),
            provided_start.hour(),
            provided_start.minute() - (provided_start.minute() % 5),
            0,
        )
        .unwrap();

    let expected_bucket_count: usize = (expected_delta.num_minutes() / 5).try_into().unwrap();
    assert_eq!(stats.common_.start_date, expected_start);
    assert_eq!(stats.common_.end_date, expected_end);
    assert_eq!(stats.common_.period, StatisticsPeriod::FiveMinutes);
    assert!(stats.data.success_count.is_some());
    assert!(stats.data.failure_count.is_some());
    assert_eq!(
        stats.data.success_count.unwrap().len(),
        expected_bucket_count
    );
    assert_eq!(
        stats.data.failure_count.unwrap().len(),
        expected_bucket_count
    );
}

#[tokio::test]
async fn test_app_attempt_fails_when_out_of_bounds() {
    let (client, _jh) = start_svix_server().await;

    let expected_delta = chrono::Duration::weeks(100);
    let now = Utc::now();
    let provided_end = now;
    let provided_start = now - expected_delta;

    let app = create_test_app(&client, "app_attempt_stats")
        .await
        .unwrap()
        .id;

    let _ = client
        .get::<IgnoredResponse>(
            &format!(
                "api/v1/stats/app/{app}/attempt/?startDate={}&endDate={}",
                dt_to_string(provided_start),
                dt_to_string(provided_end)
            ),
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_app_attempt_fails_when_start_end_reversed() {
    let (client, _jh) = start_svix_server().await;

    let expected_delta = chrono::Duration::hours(12);
    let now = Utc::now();
    let provided_end = now;
    // Adding the delta instead of subtracting here to make start > end.
    let provided_start = now + expected_delta;

    let app = create_test_app(&client, "app_attempt_stats")
        .await
        .unwrap()
        .id;

    let _ = client
        .get::<IgnoredResponse>(
            &format!(
                "api/v1/stats/app/{app}/attempt/?startDate={}&endDate={}",
                dt_to_string(provided_start),
                dt_to_string(provided_end)
            ),
            StatusCode::UNPROCESSABLE_ENTITY,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_app_attempt_bound_end_time_to_now() {
    let (client, _jh) = start_svix_server().await;

    let expected_delta = chrono::Duration::hours(1);
    let now = Utc::now();
    // 1h in the future
    let provided_end = now + expected_delta;
    // 1h in the past
    let provided_start = now - expected_delta;

    let app = create_test_app(&client, "app_attempt_stats")
        .await
        .unwrap()
        .id;

    let stats = client
        .get::<AttemptStatisticsResponse>(
            &format!(
                "api/v1/stats/app/{app}/attempt/?startDate={}&endDate={}",
                dt_to_string(provided_start),
                dt_to_string(provided_end)
            ),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // the end should be adjusted to use "now."
    let expected_end = Utc
        .with_ymd_and_hms(
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute() - (now.minute() % 5),
            0,
        )
        .unwrap();
    let expected_start = Utc
        .with_ymd_and_hms(
            provided_start.year(),
            provided_start.month(),
            provided_start.day(),
            provided_start.hour(),
            provided_start.minute() - (provided_start.minute() % 5),
            0,
        )
        .unwrap();

    let expected_bucket_count: usize = (expected_delta.num_minutes() / 5).try_into().unwrap();
    assert_eq!(stats.common_.start_date, expected_start);
    assert_eq!(stats.common_.end_date, expected_end);
    assert_eq!(stats.common_.period, StatisticsPeriod::FiveMinutes);
    assert!(stats.data.success_count.is_some());
    assert!(stats.data.failure_count.is_some());
    assert_eq!(
        stats.data.success_count.unwrap().len(),
        expected_bucket_count
    );
    assert_eq!(
        stats.data.failure_count.unwrap().len(),
        expected_bucket_count
    );
}
