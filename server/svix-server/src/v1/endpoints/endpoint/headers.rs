use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::ActiveModelTrait;
use svix_server_derive::aide_annotate;

use super::{EndpointHeadersIn, EndpointHeadersOut, EndpointHeadersPatchIn};
use crate::{
    core::permissions,
    db::models::endpoint,
    error::{HttpError, Result},
    v1::utils::{ApplicationEndpointPath, ModelIn, NoContent, ValidatedJson},
    AppState,
};

/// Get the additional headers to be sent with the webhook
#[aide_annotate(op_id = "v1.endpoint.get-headers")]
pub(super) async fn get_endpoint_headers(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
) -> Result<Json<EndpointHeadersOut>> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id, endpoint_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    match endp.headers {
        Some(h) => Ok(Json(h.into())),
        None => Ok(Json(EndpointHeadersOut::default())),
    }
}

/// Set the additional headers to be sent with the webhook
#[aide_annotate(op_id = "v1.endpoint.update-headers")]
pub(super) async fn update_endpoint_headers(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
    ValidatedJson(data): ValidatedJson<EndpointHeadersIn>,
) -> Result<NoContent> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut endp: endpoint::ActiveModel = endp.into();
    data.update_model(&mut endp);
    endp.update(db).await?;

    Ok(NoContent)
}

/// Partially set the additional headers to be sent with the webhook
#[aide_annotate(op_id = "v1.endpoint.patch-headers")]
pub(super) async fn patch_endpoint_headers(
    State(AppState { ref db, .. }): State<AppState>,
    Path(ApplicationEndpointPath { endpoint_id, .. }): Path<ApplicationEndpointPath>,
    permissions::Application { app }: permissions::Application,
    ValidatedJson(data): ValidatedJson<EndpointHeadersPatchIn>,
) -> Result<NoContent> {
    let endp = endpoint::Entity::secure_find_by_id_or_uid(app.id.clone(), endpoint_id)
        .one(db)
        .await?
        .ok_or_else(|| HttpError::not_found(None, None))?;

    let mut endp: endpoint::ActiveModel = endp.into();
    data.update_model(&mut endp);
    endp.update(db).await?;

    Ok(NoContent)
}

#[cfg(test)]
mod tests {

    use std::collections::{HashMap, HashSet};

    use sea_orm::ActiveValue::Set;

    use crate::{
        core::types::{EndpointHeaders, EndpointHeadersPatch},
        db::models::endpoint,
        v1::{
            endpoints::endpoint::{EndpointHeadersOut, EndpointHeadersPatchIn},
            utils::ModelIn,
        },
    };

    #[test]
    fn test_into_endpoint_headers_out() {
        let ep = EndpointHeaders(HashMap::from([
            ("x-non-sensitive".to_owned(), "all-clear".to_owned()),
            ("authorization".to_owned(), "should-be-omitted".to_owned()),
        ]));

        let epo: EndpointHeadersOut = ep.into();
        assert_eq!(
            HashMap::from([("x-non-sensitive".to_owned(), "all-clear".to_owned())]),
            epo.headers
        );
        assert_eq!(HashSet::from(["authorization".to_owned()]), epo.sensitive);
    }

    #[test]
    fn test_patch_endpoint_model_update() {
        let existing_hdrs = EndpointHeaders(HashMap::from([
            ("x-1".to_owned(), "123".to_owned()),
            ("x-2".to_owned(), "456".to_owned()),
        ]));
        let patched_hdrs = EndpointHeadersPatchIn {
            headers: EndpointHeadersPatch(HashMap::from([
                ("x-1".to_owned(), Some("789".to_owned())),
                ("x-3".to_owned(), Some("123".to_owned())),
            ])),
        };
        let updated_hdrs = EndpointHeaders(HashMap::from([
            ("x-1".to_owned(), "789".to_owned()),
            ("x-2".to_owned(), "456".to_owned()),
            ("x-3".to_owned(), "123".to_owned()),
        ]));
        let mut model = endpoint::ActiveModel {
            headers: Set(Some(existing_hdrs)),
            ..Default::default()
        };

        patched_hdrs.update_model(&mut model);
        assert_eq!(model.headers, Set(Some(updated_hdrs)));
    }
}
