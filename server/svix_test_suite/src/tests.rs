use crate::{fetch_env_vars, request_util};
use reqwest::StatusCode;

#[tokio::test]
async fn crd_application() {
    let (uri, secret) = fetch_env_vars();
    let (uri, secret) = (uri.0, secret.0);

    let (code, resp_create) =
        request_util::create_application(&uri, &secret, "crd_application_test_app")
            .await
            .unwrap();
    assert!(code.is_success());

    let resp_create = resp_create.unwrap();
    assert_eq!(&resp_create.name, "crd_application_test_app");

    let (code, resp_read) = request_util::read_application(&uri, &secret, &resp_create.id)
        .await
        .unwrap();

    assert!(code.is_success());

    let resp_read = resp_read.unwrap();
    assert_eq!(&resp_create, &resp_read);

    let (code, _) = request_util::delete_application(&uri, &secret, &resp_create.id)
        .await
        .unwrap();

    assert!(code.is_success());

    let (code, _) = request_util::read_application(&uri, &secret, &resp_create.id)
        .await
        .unwrap();

    assert_eq!(code, StatusCode::NOT_FOUND);
}
