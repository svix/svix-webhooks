use std::collections::HashSet;

use axum::{
    extract::{Extension, Path},
    Json,
};
use hyper::StatusCode;
use sea_orm::{entity::prelude::*, ActiveValue::Set, QueryOrder};
use sea_orm::{ActiveModelTrait, DatabaseConnection, QuerySelect};

use super::{EndpointIn, EndpointOut};
use crate::{
    core::{
        security::AuthenticatedApplication,
        types::{
            ApplicationIdOrUid, EndpointId, EndpointIdOrUid, EndpointSecret, EventTypeName,
            EventTypeNameSet, OrganizationId,
        },
    },
    db::models::{endpoint, eventtype},
    error::{HttpError, Result},
    v1::utils::{
        EmptyResponse, ListResponse, ModelIn, ModelOut, Pagination, ValidatedJson, ValidatedQuery,
    },
};
use hack::EventTypeNameResult;

pub(super) async fn list_endpoints(
    Extension(ref db): Extension<DatabaseConnection>,
    pagination: ValidatedQuery<Pagination<EndpointId>>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<ListResponse<EndpointOut>>> {
    let limit = pagination.limit;
    let iterator = pagination.iterator.clone();

    let mut query = endpoint::Entity::secure_find(app.id)
        .order_by_asc(endpoint::Column::Id)
        .limit(limit + 1);

    if let Some(iterator) = iterator {
        query = query.filter(endpoint::Column::Id.gt(iterator))
    }

    Ok(Json(EndpointOut::list_response(
        query.all(db).await?.into_iter().map(|x| x.into()).collect(),
        limit as usize,
    )))
}

pub(super) async fn create_endpoint(
    Extension(ref db): Extension<DatabaseConnection>,
    ValidatedJson(data): ValidatedJson<EndpointIn>,
    AuthenticatedApplication { permissions, app }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EndpointOut>)> {
    if let Some(ref event_types_ids) = data.event_types_ids {
        validate_event_types(db, event_types_ids, &permissions.org_id).await?;
    }

    let endp = if data.key.is_some() {
        endpoint::ActiveModel {
            app_id: Set(app.id),
            ..data.into()
        }
    } else {
        endpoint::ActiveModel {
            app_id: Set(app.id),
            key: Set(EndpointSecret::generate()?),
            ..data.into()
        }
    };
    let ret = endp.insert(db).await?;
    Ok((StatusCode::CREATED, Json(ret.into())))
}

pub(super) async fn get_endpoint(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<Json<EndpointOut>> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id, endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;
    Ok(Json(endp.into()))
}

pub(super) async fn update_endpoint(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    ValidatedJson(data): ValidatedJson<EndpointIn>,
    AuthenticatedApplication { permissions, app }: AuthenticatedApplication,
) -> Result<Json<EndpointOut>> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    if let Some(ref event_types_ids) = data.event_types_ids {
        validate_event_types(db, event_types_ids, &permissions.org_id).await?;
    }

    let mut app: endpoint::ActiveModel = endp.into();
    data.update_model(&mut app);

    let ret = app.update(db).await?;
    Ok(Json(ret.into()))
}

pub(super) async fn delete_endpoint(
    Extension(ref db): Extension<DatabaseConnection>,
    Path((_app_id, endp_id)): Path<(ApplicationIdOrUid, EndpointIdOrUid)>,
    AuthenticatedApplication {
        permissions: _,
        app,
    }: AuthenticatedApplication,
) -> Result<(StatusCode, Json<EmptyResponse>)> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endp_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut app: endpoint::ActiveModel = endp.into();
    app.deleted = Set(true);
    app.uid = Set(None); // We don't want deleted UIDs to clash
    app.update(db).await?;
    Ok((StatusCode::NO_CONTENT, Json(EmptyResponse {})))
}

/// This module is here so that our Result override doesn't conflict
mod hack {
    use sea_orm::FromQueryResult;

    use crate::core::types::EventTypeName;

    #[derive(Debug, FromQueryResult)]
    pub struct EventTypeNameResult {
        pub name: EventTypeName,
    }
}

async fn validate_event_types(
    db: &DatabaseConnection,
    event_types_ids: &EventTypeNameSet,
    org_id: &OrganizationId,
) -> Result<()> {
    let event_types: Vec<EventTypeNameResult> = eventtype::Entity::secure_find(org_id.clone())
        .filter(eventtype::Column::Deleted.eq(false))
        .select_only()
        .column(eventtype::Column::Name)
        .into_model::<EventTypeNameResult>()
        .all(db)
        .await?;
    let event_types: HashSet<EventTypeName> =
        HashSet::from_iter(event_types.into_iter().map(|x| x.name));
    let missing: Vec<&EventTypeName> = event_types_ids
        .0
        .iter()
        .filter(|x| !event_types.contains(x))
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(HttpError::unprocessable_entity(
            Some("value_error".to_owned()),
            Some(format!(
                "The following type names don't exist: {:?}",
                missing
            )),
        )
        .into())
    }
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    use crate::{
        core::types::EndpointUid,
        test_util::start_svix_server,
        v1::{
            endpoints::{
                application::tests::{create_test_app, delete_test_app},
                endpoint::{
                    tests::{
                        delete_endpoint, endpoint_in, get_404, get_endpoint, post_endpoint_default,
                        post_endpoint_in,
                    },
                    EndpointOut,
                },
            },
            utils::ListResponse,
        },
    };

    #[tokio::test]
    #[cfg_attr(not(feature = "integration_testing"), ignore)]
    async fn test_crud() {
        let (client, _jh) = start_svix_server();

        const APP_NAME_1: &str = "v1EndpointCrudTestApp1";
        const APP_NAME_2: &str = "v1EndpointCrudTestApp2";

        const EP_URI_APP_1_EP_1_VER_1: &str = "http://v1Endpointcrudtestapp1ep1ver1.test";
        const EP_URI_APP_1_EP_1_VER_2: &str = "http://v1EndpointCrudTestApp1Ep1Ver2.test";
        const EP_URI_APP_1_EP_2: &str = "http://v1EndpointCrudTestApp1Ep2.test";
        const EP_URI_APP_2_EP_1: &str = "http://v1EndpointCrudTestApp2Ep1.test";
        const EP_URI_APP_2_EP_2: &str = "http://v1EndpointCrudTestApp2Ep2.test";

        let app_1 = create_test_app(&client, APP_NAME_1).await.unwrap();
        let app_2 = create_test_app(&client, APP_NAME_2).await.unwrap();

        // CREATE
        let app_1_ep_1 = post_endpoint_default(&client, &app_1, EP_URI_APP_1_EP_1_VER_1)
            .await
            .unwrap();
        assert_eq!(app_1_ep_1.url, EP_URI_APP_1_EP_1_VER_1);
        assert_eq!(app_1_ep_1.version, 1);

        let app_1_ep_2 = post_endpoint_default(&client, &app_1, EP_URI_APP_1_EP_2)
            .await
            .unwrap();
        assert_eq!(app_1_ep_2.url, EP_URI_APP_1_EP_2);
        assert_eq!(app_1_ep_2.version, 1);

        let app_2_ep_1 = post_endpoint_default(&client, &app_2, EP_URI_APP_2_EP_1)
            .await
            .unwrap();
        assert_eq!(app_2_ep_1.url, EP_URI_APP_2_EP_1);
        assert_eq!(app_2_ep_1.version, 1);

        let app_2_ep_2 = post_endpoint_default(&client, &app_2, EP_URI_APP_2_EP_2)
            .await
            .unwrap();
        assert_eq!(app_2_ep_2.url, EP_URI_APP_2_EP_2);
        assert_eq!(app_2_ep_2.version, 1);

        // READ

        // Can read from correct app
        assert_eq!(
            get_endpoint(&client, &app_1, &app_1_ep_1.id).await.unwrap(),
            app_1_ep_1
        );
        assert_eq!(
            get_endpoint(&client, &app_1, &app_1_ep_2.id).await.unwrap(),
            app_1_ep_2
        );
        assert_eq!(
            get_endpoint(&client, &app_2, &app_2_ep_1.id).await.unwrap(),
            app_2_ep_1
        );
        assert_eq!(
            get_endpoint(&client, &app_2, &app_2_ep_2.id).await.unwrap(),
            app_2_ep_2
        );

        // Can't read from incorrect app
        get_404(&client, &app_2, &app_1_ep_1.id).await.unwrap();
        get_404(&client, &app_2, &app_1_ep_2.id).await.unwrap();
        get_404(&client, &app_1, &app_2_ep_1.id).await.unwrap();
        get_404(&client, &app_1, &app_2_ep_2.id).await.unwrap();

        // UPDATE
        let app_1_ep_1_id = app_1_ep_1.id;
        let app_1_ep_1: EndpointOut = client
            .put(
                &format!("api/v1/app/{}/endpoint/{}/", app_1, app_1_ep_1_id),
                endpoint_in(EP_URI_APP_1_EP_1_VER_2),
                StatusCode::OK,
            )
            .await
            .unwrap();
        assert_eq!(app_1_ep_1.url, EP_URI_APP_1_EP_1_VER_2);

        // CONFIRM UPDATE
        assert_eq!(
            get_endpoint(&client, &app_1, &app_1_ep_1_id).await.unwrap(),
            app_1_ep_1
        );

        // LIST
        let list_app_1: ListResponse<EndpointOut> = client
            .get(&format!("api/v1/app/{}/endpoint/", &app_1), StatusCode::OK)
            .await
            .unwrap();
        assert_eq!(list_app_1.data.len(), 2);
        assert!(list_app_1.data.contains(&app_1_ep_1));
        assert!(list_app_1.data.contains(&app_1_ep_2));

        let list_app_2: ListResponse<EndpointOut> = client
            .get(&format!("api/v1/app/{}/endpoint/", &app_2), StatusCode::OK)
            .await
            .unwrap();
        assert_eq!(list_app_2.data.len(), 2);
        assert!(list_app_2.data.contains(&app_2_ep_1));
        assert!(list_app_2.data.contains(&app_2_ep_2));

        // DELETE
        delete_endpoint(&client, &app_1, &app_1_ep_1.id)
            .await
            .unwrap();
        delete_endpoint(&client, &app_1, &app_1_ep_2.id)
            .await
            .unwrap();
        delete_endpoint(&client, &app_2, &app_2_ep_1.id)
            .await
            .unwrap();
        delete_endpoint(&client, &app_2, &app_2_ep_2.id)
            .await
            .unwrap();

        // CONFIRM DELETION
        get_404(&client, &app_1, &app_1_ep_1.id).await.unwrap();
        get_404(&client, &app_1, &app_1_ep_2.id).await.unwrap();
        get_404(&client, &app_2, &app_2_ep_1.id).await.unwrap();
        get_404(&client, &app_2, &app_2_ep_2.id).await.unwrap();
    }

    /// Tests that there is at most one endpoint with a single UID for all endpoints associated with
    /// any application
    #[tokio::test]
    #[cfg_attr(not(feature = "integration_testing"), ignore)]
    async fn test_uid() {
        let (client, _jh) = start_svix_server();

        const APP_NAME_1: &str = "v1EndpointUidTestApp1";
        const APP_NAME_2: &str = "v1EndpointUidTestApp2";

        const EP_URI_APP_1_EP_1: &str = "http://v1EndpointUidTestApp1Ep1.test";
        const EP_URI_APP_1_EP_2: &str = "http://v1EndpointUidTestApp1Ep2.test";
        const EP_URI_APP_2: &str = "http://v1EndpointUidTestApp2Ep1.test";

        const DUPLICATE_UID: &str = "test_uid";

        // Same App

        // Double Create -- on creation, it should return an error if identical UIDs are used for
        // endpoints in the same app
        let app_id = create_test_app(&client, APP_NAME_1).await.unwrap();
        let uid = EndpointUid(DUPLICATE_UID.to_owned());

        let mut ep_1 = endpoint_in(EP_URI_APP_1_EP_1);
        ep_1.uid = Some(uid.clone());

        let mut ep_2 = endpoint_in(EP_URI_APP_1_EP_2);
        ep_2.uid = Some(uid.clone());

        let ep_1 = post_endpoint_in(&client, &app_id, ep_1).await.unwrap();
        assert!(post_endpoint_in(&client, &app_id, ep_2).await.is_err());

        // Update One to Existing -- on update it should return an error if attempting to change
        // the UID to that of an existing endpoint associated with the same app
        let ep_2 = post_endpoint_default(&client, &app_id, EP_URI_APP_1_EP_2)
            .await
            .unwrap();

        let mut ep_2_with_invalid_uid = endpoint_in(EP_URI_APP_1_EP_2);
        ep_2_with_invalid_uid.uid = Some(uid.clone());

        assert!(client
            .put::<_, EndpointOut>(
                &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_2.id),
                ep_2_with_invalid_uid,
                StatusCode::OK,
            )
            .await
            .is_err());

        // Update One to Identical -- however it should not return an error if updating the
        // existing endpoint to one with the same UID
        let mut ep_1_with_duplicate_id = endpoint_in(EP_URI_APP_1_EP_1);
        ep_1_with_duplicate_id.uid = Some(uid.clone());

        let ep_1_updated = client
            .put::<_, EndpointOut>(
                &format!("api/v1/app/{}/endpoint/{}/", app_id, ep_1.id),
                ep_1_with_duplicate_id,
                StatusCode::OK,
            )
            .await
            .unwrap();
        assert_eq!(ep_1.id, ep_1_updated.id);
        assert_eq!(ep_1.uid, ep_1_updated.uid);

        // Delete One then Create One -- UIDs may no be reused after deletion
        delete_endpoint(&client, &app_id, &ep_1.id).await.unwrap();
        delete_endpoint(&client, &app_id, &ep_2.id).await.unwrap();

        let mut ep_1 = endpoint_in(EP_URI_APP_1_EP_1);
        ep_1.uid = Some(uid.clone());
        assert!(post_endpoint_in(&client, APP_NAME_1, ep_1).await.is_err());

        delete_test_app(&client, app_id).await.unwrap();

        // Different App -- however if they are associated with different applications, identical
        // UIDs are valid
        let app_1 = create_test_app(&client, APP_NAME_1).await.unwrap();
        let app_2 = create_test_app(&client, APP_NAME_2).await.unwrap();

        let mut ep_1 = endpoint_in(EP_URI_APP_1_EP_1);
        ep_1.uid = Some(uid.clone());

        let mut ep_2 = endpoint_in(EP_URI_APP_2);
        ep_2.uid = Some(uid.clone());

        let _ = post_endpoint_in(&client, &app_1, ep_1).await.unwrap();
        let _ = post_endpoint_in(&client, &app_2, ep_2).await.unwrap();
    }
}
