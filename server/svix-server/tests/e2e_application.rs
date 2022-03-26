// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use reqwest::StatusCode;

use svix_server::{v1::endpoints::application::ApplicationOut, v1::utils::ListResponse};

mod utils;

use utils::{common_calls::application_in, start_svix_server, IgnoredResponse};

#[tokio::test]
async fn test_crud() {
    let (client, _jh) = start_svix_server();

    const APP_NAME_1_1: &str = "v1ApplicationCrudTest11";
    const APP_NAME_1_2: &str = "v1ApplicationCrudTest12";
    const APP_NAME_2_1: &str = "v1ApplicationCrudTest21";
    const APP_NAME_2_2: &str = "v1ApplicationCrudTest22";

    // CREATE
    let app_1: ApplicationOut = client
        .post(
            "api/v1/app/",
            application_in(APP_NAME_1_1),
            StatusCode::CREATED,
        )
        .await
        .unwrap();
    assert_eq!(app_1.name, APP_NAME_1_1);

    let app_2: ApplicationOut = client
        .post(
            "api/v1/app/",
            application_in(APP_NAME_2_1),
            StatusCode::CREATED,
        )
        .await
        .unwrap();
    assert_eq!(app_2.name, APP_NAME_2_1);

    // READ
    assert_eq!(
        client
            .get::<ApplicationOut>(&format!("api/v1/app/{}/", app_1.id), StatusCode::OK)
            .await
            .unwrap(),
        app_1
    );

    assert_eq!(
        client
            .get::<ApplicationOut>(&format!("api/v1/app/{}/", app_2.id), StatusCode::OK,)
            .await
            .unwrap(),
        app_2
    );

    // UPDATE
    let app_1_id = app_1.id;
    let app_1: ApplicationOut = client
        .put(
            &format!("api/v1/app/{}", app_1_id),
            application_in(APP_NAME_1_2),
            StatusCode::OK,
        )
        .await
        .unwrap();

    let app_2_id = app_2.id;
    let app_2: ApplicationOut = client
        .put(
            &format!("api/v1/app/{}", app_2_id),
            application_in(APP_NAME_2_2),
            StatusCode::OK,
        )
        .await
        .unwrap();

    // CONFIRM UPDATE
    assert_eq!(
        client
            .get::<ApplicationOut>(&format!("api/v1/app/{}/", app_1_id), StatusCode::OK,)
            .await
            .unwrap(),
        app_1
    );

    assert_eq!(
        client
            .get::<ApplicationOut>(&format!("api/v1/app/{}/", app_2_id), StatusCode::OK,)
            .await
            .unwrap(),
        app_2
    );

    // DELETE
    let _: IgnoredResponse = client
        .delete(&format!("api/v1/app/{}/", app_1.id), StatusCode::NO_CONTENT)
        .await
        .unwrap();
    let _: IgnoredResponse = client
        .delete(&format!("api/v1/app/{}/", app_2.id), StatusCode::NO_CONTENT)
        .await
        .unwrap();

    // CONFIRM DELETION
    let _: IgnoredResponse = client
        .get(&format!("api/v1/app/{}/", app_1.id), StatusCode::NOT_FOUND)
        .await
        .unwrap();
    let _: IgnoredResponse = client
        .get(&format!("api/v1/app/{}/", app_2.id), StatusCode::NOT_FOUND)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_list() {
    let (client, _jh) = start_svix_server();

    const APP_NAME_1: &str = "v1ApplicationCrudTest1";
    const APP_NAME_2: &str = "v1ApplicationCrudTest2";

    // CREATE
    let app_1: ApplicationOut = client
        .post(
            "api/v1/app/",
            application_in(APP_NAME_1),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let app_2: ApplicationOut = client
        .post(
            "api/v1/app/",
            application_in(APP_NAME_2),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    let list = client
        .get::<ListResponse<ApplicationOut>>("api/v1/app/", StatusCode::OK)
        .await
        .unwrap();

    assert_eq!(list.data.len(), 2);
    assert_eq!(&app_1, list.data.get(0).unwrap());
    assert_eq!(&app_2, list.data.get(1).unwrap());
}
