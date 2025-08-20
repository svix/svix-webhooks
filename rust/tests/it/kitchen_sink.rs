use js_option::JsOption;
use std::{collections::HashSet, time::Duration};
use svix::{
    api::{ApplicationIn, EndpointIn, EndpointPatch, EventTypeIn},
    error::Error,
};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::utils::test_client::TestClientBuilder;

fn check_for_conflict(e: Error) {
    match e {
        Error::Http(e) => {
            assert_eq!(
                e.status,
                http02::StatusCode::CONFLICT,
                "conflicts are expected but other statuses are not"
            );
        }
        _ => panic!("unexpected error: {e}"),
    }
}

// Opt-in with `cargo test --ignored`
#[ignore]
#[tokio::test]
async fn test_endpoint_crud() {
    let client = TestClientBuilder::new().build();
    let client = client.client;

    let app = client
        .application()
        .create(
            ApplicationIn {
                name: "app".to_string(),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    if let Err(e) = client
        .event_type()
        .create(
            EventTypeIn {
                name: String::from("event.started"),
                description: String::from("Something started"),
                ..Default::default()
            },
            None,
        )
        .await
    {
        check_for_conflict(e);
    }

    if let Err(e) = client
        .event_type()
        .create(
            EventTypeIn {
                name: String::from("event.ended"),
                description: String::from("Something ended"),
                ..Default::default()
            },
            None,
        )
        .await
    {
        check_for_conflict(e);
    }

    let ep = client
        .endpoint()
        .create(
            app.id.clone(),
            EndpointIn {
                channels: Some(vec![String::from("ch0"), String::from("ch1")]),
                url: String::from("https://example.svix.com/"),
                ..Default::default()
            },
            None,
        )
        .await
        .unwrap();

    let want_channels: HashSet<_> = [String::from("ch0"), String::from("ch1")]
        .into_iter()
        .collect();
    let got_channels = ep.channels.clone().unwrap().into_iter().collect();
    assert_eq!(want_channels, got_channels);
    assert_eq!(0, ep.filter_types.unwrap_or_default().len());

    let ep_patched = client
        .endpoint()
        .patch(
            app.id.clone(),
            ep.id.clone(),
            EndpointPatch {
                filter_types: JsOption::Some(vec![
                    String::from("event.started"),
                    String::from("event.ended"),
                ]),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let want_filter_types: HashSet<_> =
        [String::from("event.started"), String::from("event.ended")]
            .into_iter()
            .collect();
    let got_channels = ep_patched.channels.clone().unwrap().into_iter().collect();
    let got_filter_types = ep_patched
        .filter_types
        .clone()
        .unwrap()
        .into_iter()
        .collect();
    assert_eq!(want_channels, got_channels);
    assert_eq!(want_filter_types, got_filter_types);

    // Should complete without error if the deserialization handles empty bodies
    // correctly.
    client
        .endpoint()
        .delete(app.id.clone(), ep.id)
        .await
        .unwrap();

    client.application().delete(app.id).await.unwrap()
}

#[tokio::test]
async fn test_default_retries() {
    let mock_server: MockServer = MockServer::start().await;

    Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/api/v1/app"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/api/v1/app"))
        .and(wiremock::matchers::header("svix-retry-count", 1))
        .and(wiremock::matchers::header_exists("svix-req-id"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/api/v1/app"))
        .and(wiremock::matchers::header("svix-retry-count", 2))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    let t0 = std::time::Instant::now();
    let client = TestClientBuilder::new()
        .url(mock_server.uri())
        .token("test".to_string())
        .build()
        .client;

    let app = client
        .application()
        .create(
            ApplicationIn {
                name: "app".to_string(),
                ..Default::default()
            },
            None,
        )
        .await;
    assert!(app.is_err());

    let diff = std::time::Instant::now() - t0;
    assert!(diff.as_millis() >= 60);

    mock_server.verify().await;
}

#[tokio::test]
async fn test_custom_retries() {
    let mock_server: MockServer = MockServer::start().await;
    let num_retries = 6;

    Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/api/v1/app"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    for i in 1..=num_retries {
        Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/api/v1/app"))
            .and(wiremock::matchers::header("svix-retry-count", i))
            .respond_with(ResponseTemplate::new(500))
            .up_to_n_times(1)
            .expect(1)
            .mount(&mock_server)
            .await;
    }

    let t0 = std::time::Instant::now();
    let client = TestClientBuilder::new()
        .url(mock_server.uri())
        .token("test".to_string())
        .retries(num_retries)
        .build()
        .client;

    let app = client
        .application()
        .create(
            ApplicationIn {
                name: "app".to_string(),
                ..Default::default()
            },
            None,
        )
        .await;
    assert!(app.is_err());

    let diff = std::time::Instant::now() - t0;
    let expected: u32 = (1..=num_retries).map(|x| 20 * x).sum();
    assert!(diff.as_millis() >= u128::from(expected));

    mock_server.verify().await;
}

#[tokio::test]
async fn test_custom_retry_schedule() {
    let mock_server: MockServer = MockServer::start().await;
    let retry_schedule_in_ms = [50, 100, 200, 400];
    let retry_schedule = retry_schedule_in_ms.map(Duration::from_millis).into();

    Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/api/v1/app"))
        .respond_with(ResponseTemplate::new(500))
        .up_to_n_times(1)
        .expect(1)
        .mount(&mock_server)
        .await;

    for i in 1..=retry_schedule_in_ms.len() {
        Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/api/v1/app"))
            .and(wiremock::matchers::header("svix-retry-count", i))
            .respond_with(ResponseTemplate::new(500))
            .up_to_n_times(1)
            .expect(1)
            .mount(&mock_server)
            .await;
    }

    let t0 = std::time::Instant::now();
    let client = TestClientBuilder::new()
        .url(mock_server.uri())
        .token("test".to_string())
        .retry_schedule(retry_schedule)
        .build()
        .client;

    let app = client
        .application()
        .create(
            ApplicationIn {
                name: "app".to_string(),
                ..Default::default()
            },
            None,
        )
        .await;
    assert!(app.is_err());

    let diff = std::time::Instant::now() - t0;
    let expected: u64 = retry_schedule_in_ms.iter().sum();
    assert!(diff.as_millis() >= u128::from(expected));

    mock_server.verify().await;
}
