//! Test module for worker functionality that depends on external networking and test utilities.
//! As such they are included with integration tests for organizational purposes.
use std::{net::TcpListener, sync::Arc, time::Duration};

use axum::extract::State;
use http::StatusCode;
use svix_server::v1::{
    endpoints::{attempt::MessageAttemptOut, endpoint::EndpointOut},
    utils::ListResponse,
};
use tokio::sync::Mutex;

mod utils;
use utils::{
    common_calls::{create_test_app, create_test_endpoint, create_test_message},
    get_default_test_config, run_with_retries, start_svix_server, start_svix_server_with_cfg,
};

/// Runs a full Axum server with two endoints. The first endpoint redirects to the second endpoint
/// while the second endpoint records whether it has been visited. This is such that we can check
/// that no redirection is taken by the Svix worker's `reqwest::Client`
struct RedirectionVisitReportingReceiver {
    pub base_uri: String,
    pub jh: tokio::task::JoinHandle<()>,
    pub has_been_visited: Arc<Mutex<bool>>,
}

#[derive(Clone)]
struct RedirectionVisitReportingState {
    has_been_visited: Arc<Mutex<bool>>,
    resp_with: axum::http::StatusCode,
}

impl RedirectionVisitReportingReceiver {
    pub fn start(resp_with: axum::http::StatusCode) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base_uri = format!("http://{}", listener.local_addr().unwrap());

        let has_been_visited = Arc::new(Mutex::new(false));

        let routes = axum::Router::new()
            .route(
                "/first/",
                axum::routing::post(redirecting_receiver_route).get(redirecting_receiver_route),
            )
            .route(
                "/second/",
                axum::routing::post(visit_reporting_receiver_route)
                    .get(visit_reporting_receiver_route),
            )
            .with_state(RedirectionVisitReportingState {
                has_been_visited: has_been_visited.clone(),
                resp_with,
            })
            .into_make_service();

        let jh = tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(routes)
                .await
                .unwrap();
        });

        RedirectionVisitReportingReceiver {
            base_uri,
            jh,
            has_been_visited,
        }
    }
}

async fn redirecting_receiver_route() -> axum::response::Redirect {
    axum::response::Redirect::permanent("/second/")
}

async fn visit_reporting_receiver_route(
    State(RedirectionVisitReportingState {
        has_been_visited: visited,
        resp_with,
    }): State<RedirectionVisitReportingState>,
) -> StatusCode {
    *visited.lock().await = true;
    resp_with
}

// The worker has
#[tokio::test]
async fn test_no_redirects_policy() {
    let (client, _jh) = start_svix_server().await;
    let receiver = RedirectionVisitReportingReceiver::start(StatusCode::OK);

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    let _ep_id = create_test_endpoint(&client, &app_id, &format!("{}/first/", receiver.base_uri))
        .await
        .unwrap()
        .id;
    let msg_id = create_test_message(&client, &app_id, serde_json::json!({}))
        .await
        .unwrap()
        .id;

    run_with_retries(|| async {
        let attempts: ListResponse<MessageAttemptOut> = client
            .get(
                &format!("api/v1/app/{app_id}/attempt/msg/{msg_id}/"),
                StatusCode::OK,
            )
            .await
            .unwrap();

        let attempt = attempts.data.get(0);

        if let Some(attempt) = attempt {
            assert_eq!(attempt.response_status_code, 308);
            Ok(())
        } else {
            anyhow::bail!("No attempt found");
        }
    })
    .await
    .unwrap();

    // Assert that the second endpoint has not been visited
    assert!(!*receiver.has_been_visited.lock().await);

    receiver.jh.abort();
}

/// This tests that endpoints are successfully disabled after the retry schedule is exhausted
/// multiple times without intermittent success over a period exceeding the grace period. So the
/// tests don't take too long, these grace period and expiration period will be reconfigured to be
/// on the order of seconds
#[tokio::test]
async fn test_endpoint_disable_on_repeated_failure() {
    let mut cfg = get_default_test_config();

    if !matches!(cfg.cache_type, svix_server::cfg::CacheType::None) {
        cfg.retry_schedule = vec![];
        cfg.endpoint_failure_disable_after = Duration::from_secs(2);

        let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

        let app_id = create_test_app(&client, "app").await.unwrap().id;
        let ep_id = create_test_endpoint(&client, &app_id, "http://bad.url/")
            .await
            .unwrap()
            .id;

        let _msg_id = create_test_message(&client, &app_id, serde_json::json!({}))
            .await
            .unwrap()
            .id;

        tokio::time::sleep(Duration::from_millis(2_500)).await;

        let _msg_id = create_test_message(&client, &app_id, serde_json::json!({}))
            .await
            .unwrap()
            .id;

        run_with_retries(|| async {
            let ep: EndpointOut = client
                .get(
                    &format!("api/v1/app/{app_id}/endpoint/{ep_id}/"),
                    StatusCode::OK,
                )
                .await
                .unwrap();

            if !ep.ep.disabled {
                anyhow::bail!("Endpoint not disabled")
            } else {
                Ok(())
            }
        })
        .await
        .unwrap();
    }
}

/// This tests that if a consistently failing endpoint is only tried after the expiration period
/// has been exceeded, that it will not be disabled.
#[tokio::test]
async fn test_endpoint_disable_expiration_duration() {
    let mut cfg = get_default_test_config();

    if !matches!(cfg.cache_type, svix_server::cfg::CacheType::None) {
        cfg.retry_schedule = vec![];
        cfg.endpoint_failure_disable_after = Duration::from_millis(250);

        let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

        let app_id = create_test_app(&client, "app").await.unwrap().id;
        let ep_id = create_test_endpoint(&client, &app_id, "http://bad.url/")
            .await
            .unwrap()
            .id;

        let _msg_id = create_test_message(&client, &app_id, serde_json::json!({}))
            .await
            .unwrap()
            .id;

        tokio::time::sleep(Duration::from_millis(1200)).await;

        let _msg_id = create_test_message(&client, &app_id, serde_json::json!({}))
            .await
            .unwrap()
            .id;

        // Cannot run with retries as it's not disabled by default and we are checking that it remains
        // not disabled. So another sleep is required here.
        tokio::time::sleep(Duration::from_millis(500)).await;
        let ep: EndpointOut = client
            .get(
                &format!("api/v1/app/{app_id}/endpoint/{ep_id}/"),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert!(!ep.ep.disabled);
    }
}

/// Because the endpoint disabling system requires failures, we need a test receiver that fails
/// some of the time, but not all of the time, such as to be able to test that a successful response
/// after a failure clears the cache for that endpoint.
struct SporadicallyFailingReceiver {
    pub base_uri: String,
    pub jh: tokio::task::JoinHandle<()>,
}

#[derive(Clone)]
struct SporadicallyFailingState {
    count: Arc<Mutex<u8>>,
    resp_with: (http::StatusCode, http::StatusCode),
}

impl SporadicallyFailingReceiver {
    pub fn start(resp_with: (http::StatusCode, http::StatusCode)) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base_uri = format!("http://{}", listener.local_addr().unwrap());

        let count = Arc::new(Mutex::new(0u8));

        let routes = axum::Router::new()
            .route(
                "/",
                axum::routing::post(sporadically_failing_route).get(sporadically_failing_route),
            )
            .with_state(SporadicallyFailingState { count, resp_with })
            .into_make_service();

        let jh = tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(routes)
                .await
                .unwrap();
        });

        SporadicallyFailingReceiver { base_uri, jh }
    }
}

async fn sporadically_failing_route(
    State(SporadicallyFailingState {
        count,
        resp_with: (resp_ok, resp_fail),
    }): State<SporadicallyFailingState>,
) -> StatusCode {
    let mut count = count.lock().await;
    *count += 1;

    if *count % 2 == 0 {
        resp_ok
    } else {
        resp_fail
    }
}

/// This tetss that if an endpoint succceeds, that its record is cleared in the cache and it is not
/// disabled after the grace period following a failure.
#[tokio::test]
async fn test_endpoint_disable_on_sporadic_failure() {
    let mut cfg = get_default_test_config();

    if !matches!(cfg.cache_type, svix_server::cfg::CacheType::None) {
        let receiver =
            SporadicallyFailingReceiver::start((StatusCode::OK, StatusCode::INTERNAL_SERVER_ERROR));

        cfg.retry_schedule = vec![];
        cfg.endpoint_failure_disable_after = Duration::from_secs(1);

        let (client, _jh) = start_svix_server_with_cfg(&cfg).await;

        let app_id = create_test_app(&client, "app").await.unwrap().id;
        let ep_id = create_test_endpoint(&client, &app_id, &receiver.base_uri)
            .await
            .unwrap()
            .id;

        // Fails
        let _msg_id = create_test_message(&client, &app_id, serde_json::json!({}))
            .await
            .unwrap()
            .id;

        // Sleep here to make sure the failure has been processed *before* the successful response clears
        // the key. Otherwise the set may happen after the clear and the endpoint will be disabled after
        // the second failure
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Succeeds
        let _msg_id = create_test_message(&client, &app_id, serde_json::json!({}))
            .await
            .unwrap()
            .id;

        tokio::time::sleep(Duration::from_millis(1200)).await;

        // Fails
        let _msg_id = create_test_message(&client, &app_id, serde_json::json!({}))
            .await
            .unwrap()
            .id;

        // Cannot run with retries as it's not disabled by default and we are checking that it remains
        // not disabled. So another sleep is required here.
        tokio::time::sleep(Duration::from_millis(500)).await;
        let ep: EndpointOut = client
            .get(
                &format!("api/v1/app/{app_id}/endpoint/{ep_id}/"),
                StatusCode::OK,
            )
            .await
            .unwrap();

        assert!(!ep.ep.disabled);

        receiver.jh.abort();
    }
}
