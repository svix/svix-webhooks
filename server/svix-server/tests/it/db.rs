// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

//! Configuration-dependent tests for the main database

use chrono::Utc;
use http::StatusCode;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, Set};
use svix_server::{
    core::types::{
        ApplicationId, BaseId, EndpointId, EventTypeName, MessageAttemptId,
        MessageAttemptTriggerType, MessageId, MessageStatus, OrganizationId,
    },
    db::{
        models::{application, endpoint, eventtype, message, messageattempt},
        prune_messages, wipe_org,
    },
    v1::endpoints::event_type::EventTypeOut,
};

use crate::utils::{
    common_calls::{create_test_app, create_test_endpoint, create_test_message, event_type_in},
    get_default_test_config, start_svix_server_with_cfg_and_org_id,
};

async fn test_data() -> (OrganizationId, Vec<ApplicationId>, Vec<EndpointId>) {
    let org_id = OrganizationId::new(None, None);

    let (client, jh) =
        start_svix_server_with_cfg_and_org_id(&get_default_test_config(), org_id.clone()).await;

    // Make apps
    let mut app_ids = Vec::new();
    for _ in 0..10 {
        app_ids.push(create_test_app(&client, "Test1").await.unwrap().id);
    }

    let mut endp_ids = Vec::new();
    for app_id in &app_ids {
        for _ in 0..2 {
            endp_ids.push(
                create_test_endpoint(&client, app_id, "https://bad.url")
                    .await
                    .unwrap()
                    .id,
            );
        }
    }

    for app_id in &app_ids {
        for _ in 0..2 {
            create_test_message(&client, app_id, serde_json::json!({"test": "value"}))
                .await
                .unwrap();
        }
    }

    let _: EventTypeOut = client
        .post(
            "api/v1/event-type",
            event_type_in("test", None).unwrap(),
            StatusCode::CREATED,
        )
        .await
        .unwrap();

    jh.abort();

    (org_id, app_ids, endp_ids)
}

async fn count_message_attempts(db: &DatabaseConnection, endp_id: EndpointId) -> usize {
    messageattempt::Entity::secure_find_by_endpoint(endp_id)
        .all(db)
        .await
        .unwrap()
        .len()
}

async fn count_messages(db: &DatabaseConnection, app_id: ApplicationId) -> usize {
    message::Entity::secure_find(app_id)
        .all(db)
        .await
        .unwrap()
        .len()
}

async fn count_endpoints(db: &DatabaseConnection, app_id: ApplicationId) -> usize {
    endpoint::Entity::secure_find(app_id)
        .all(db)
        .await
        .unwrap()
        .len()
}

async fn count_applications(db: &DatabaseConnection, org_id: OrganizationId) -> usize {
    application::Entity::secure_find(org_id)
        .all(db)
        .await
        .unwrap()
        .len()
}

async fn count_event_types(db: &DatabaseConnection, org_id: OrganizationId) -> usize {
    eventtype::Entity::secure_find(org_id)
        .all(db)
        .await
        .unwrap()
        .len()
}

async fn insert_message(
    db: &DatabaseConnection,
    app_id: ApplicationId,
    org_id: OrganizationId,
    timestamp: chrono::DateTime<Utc>,
) -> message::Model {
    message::ActiveModel {
        app_id: Set(app_id),
        org_id: Set(org_id),
        expiration: Set((timestamp + chrono::Duration::days(90)).into()),
        event_type: Set(EventTypeName("test.event".into())),
        created_at: Set(timestamp.into()),
        id: Set(MessageId::new(timestamp.into(), None)),
        ..message::ActiveModel::new()
    }
    .insert(db)
    .await
    .unwrap()
}

async fn insert_attempt(
    db: &DatabaseConnection,
    msg_id: MessageId,
    endp_id: EndpointId,
    timestamp: chrono::DateTime<Utc>,
) -> messageattempt::Model {
    messageattempt::ActiveModel {
        endp_id: Set(endp_id),
        msg_id: Set(msg_id),
        id: Set(MessageAttemptId::new(timestamp.into(), None)),
        status: Set(MessageStatus::Success),
        created_at: Set(timestamp.into()),
        url: Set("http://www.example.com".into()),
        response_status_code: Set(200),
        response_duration_ms: Set(100),
        response: Set("{}".into()),
        trigger_type: Set(MessageAttemptTriggerType::Scheduled),
        ..ActiveModelTrait::default()
    }
    .insert(db)
    .await
    .unwrap()
}

#[ignore]
#[tokio::test]
async fn test_wiping_organization() {
    // Make two orgs, one of which will be wiped
    let (org_id_1, app_ids_1, endp_ids_1) = test_data().await;

    let (org_id_2, app_ids_2, _) = test_data().await;

    // Wipe
    let cfg = svix_server::cfg::load().unwrap();
    wipe_org(&cfg, org_id_1.clone()).await;

    // Start asserting everything is gone for org_id_1, but not org_id_2
    let db = svix_server::db::init_db(&cfg).await;

    for endp_id in endp_ids_1 {
        assert_eq!(count_message_attempts(&db, endp_id.clone()).await, 0);
    }

    for app_id in app_ids_1 {
        assert_eq!(count_messages(&db, app_id.clone()).await, 0);
        assert_eq!(count_endpoints(&db, app_id.clone()).await, 0);
    }

    for app_id in app_ids_2 {
        assert_eq!(count_messages(&db, app_id.clone()).await, 2);
        assert_eq!(count_endpoints(&db, app_id.clone()).await, 2);
    }

    assert_eq!(count_applications(&db, org_id_1.clone()).await, 0);
    assert_eq!(count_event_types(&db, org_id_1.clone()).await, 0);

    assert_eq!(count_applications(&db, org_id_2.clone()).await, 10);
    assert_eq!(count_event_types(&db, org_id_2.clone()).await, 1);
}

#[tokio::test]
async fn test_prune_deletes_old_keeps_new() {
    let org_id = OrganizationId::new(None, None);
    let cfg = std::sync::Arc::new(get_default_test_config());

    let (client, jh) = start_svix_server_with_cfg_and_org_id(cfg.as_ref(), org_id.clone()).await;
    let app_id = create_test_app(&client, "prune-test-app").await.unwrap().id;
    let endp_id = create_test_endpoint(&client, &app_id, "https://example.com")
        .await
        .unwrap()
        .id;
    jh.abort();

    let db = svix_server::db::init_db(&cfg).await;
    let now = Utc::now();

    // Insert old messages (200 days ago) with 2 attempts each
    for i in 0..10 {
        let ts = now - chrono::Duration::days(200) + chrono::Duration::seconds(i);
        let msg = insert_message(&db, app_id.clone(), org_id.clone(), ts).await;
        for j in 0..2 {
            insert_attempt(
                &db,
                msg.id.clone(),
                endp_id.clone(),
                ts + chrono::Duration::seconds(j + 1),
            )
            .await;
        }
    }

    // Insert 2 new messages (10 days ago) with 2 attempts each
    for i in 0..10 {
        let ts = now - chrono::Duration::days(10) + chrono::Duration::seconds(i);
        let msg = insert_message(&db, app_id.clone(), org_id.clone(), ts).await;
        for j in 0..2 {
            insert_attempt(
                &db,
                msg.id.clone(),
                endp_id.clone(),
                ts + chrono::Duration::seconds(j + 1),
            )
            .await;
        }
    }

    let cutoff = now - chrono::Duration::days(100);
    prune_messages(&cfg, cutoff, 10_000).await.unwrap();

    assert_eq!(count_messages(&db, app_id.clone()).await, 10);
    assert_eq!(count_message_attempts(&db, endp_id.clone()).await, 20);
}

#[tokio::test]
async fn test_prune_with_nothing_old_is_noop() {
    let org_id = OrganizationId::new(None, None);
    let cfg = std::sync::Arc::new(get_default_test_config());

    let (client, jh) = start_svix_server_with_cfg_and_org_id(cfg.as_ref(), org_id.clone()).await;
    let app_id = create_test_app(&client, "prune-noop-app").await.unwrap().id;
    let endp_id = create_test_endpoint(&client, &app_id, "https://example.com")
        .await
        .unwrap()
        .id;
    jh.abort();

    let db = svix_server::db::init_db(&cfg).await;
    let now = Utc::now();

    // Insert new messages (10 days ago) with 1 attempt each
    for i in 0..10 {
        let ts = now - chrono::Duration::days(10) + chrono::Duration::seconds(i);
        let msg = insert_message(&db, app_id.clone(), org_id.clone(), ts).await;
        insert_attempt(
            &db,
            msg.id.clone(),
            endp_id.clone(),
            ts + chrono::Duration::seconds(1),
        )
        .await;
    }

    let cutoff = now - chrono::Duration::days(100);
    prune_messages(&cfg, cutoff, 10_000u64).await.unwrap();

    assert_eq!(count_messages(&db, app_id.clone()).await, 10);
    assert_eq!(count_message_attempts(&db, endp_id.clone()).await, 10);
}
