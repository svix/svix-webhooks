//! Test module for worker functionality that depends on external networking and test utilities.
//! As such they are included with integration tests for organizational purposes.
use std::{net::TcpListener, sync::Arc};

use axum::extract::Extension;
use http::StatusCode;
use svix_server::v1::{endpoints::attempt::MessageAttemptOut, utils::ListResponse};
use tokio::sync::Mutex;

mod utils;
use utils::{
    common_calls::{create_test_app, create_test_endpoint, create_test_message},
    run_with_retries, start_svix_server, ResponseStatusCode,
};

/// Runs a full Axum server with two endoints. The first endpoint redirects to the second endpoint
/// while the second endpoint records whether it has been visited. This is such that we can check
/// that no redirection is taken by the Svix worker's `reqwest::Client`
struct RedirectionVisitReportingReceiver {
    pub base_uri: String,
    pub jh: tokio::task::JoinHandle<()>,
    pub has_been_visited: Arc<Mutex<bool>>,
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
            .layer(Extension(has_been_visited.clone()))
            .layer(Extension(resp_with))
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
    Extension(visited): Extension<Arc<Mutex<bool>>>,
    Extension(status_code): Extension<Arc<Mutex<ResponseStatusCode>>>,
) -> StatusCode {
    *visited.lock().await = true;
    status_code.lock().await.status_code
}

// The worker has
#[tokio::test]
async fn test_no_redirects_policy() {
    let (client, _jh) = start_svix_server();
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
                &format!("api/v1/app/{}/attempt/msg/{}/", app_id, msg_id),
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

    // Assert that the second enpoint has not been visited
    assert!(!*receiver.has_been_visited.lock().await);

    receiver.jh.abort();
}
