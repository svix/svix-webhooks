//! Test module for worker functionality that depends on external networking and test utilities.
//! As such they are included with integration tests for organizational purposes.
mod utils;
use http::StatusCode;
use svix_server::v1::{endpoints::attempt::MessageAttemptOut, utils::ListResponse};
use utils::{
    common_calls::{create_test_app, create_test_endpoint, create_test_message},
    run_with_retries, start_svix_server,
};

// The worker has
#[tokio::test]
async fn test_no_redirects_policy() {
    let (client, _jh) = start_svix_server();

    let app_id = create_test_app(&client, "app1").await.unwrap().id;

    // The Svix website will redirect
    // TODO: Redirecting test receiver so the tests don't depend on external networking
    let _ep_id = create_test_endpoint(&client, &app_id, "https://svix.com")
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
}
