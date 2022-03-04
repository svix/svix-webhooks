use std::convert::TryInto;

use chrono::{DateTime, FixedOffset};
use sea_orm::{DatabaseTransaction};
use serde::{Deserialize, Serialize};

use super::{kv_def, CacheKey, CacheValue};
use crate::{
    core::types::{
        ApplicationId, ApplicationUid, EndpointHeaders, EndpointId, EndpointSecret,
        EventChannelSet, EventTypeNameSet, ExpiringSigningKeys, OrganizationId,
    },
    db::models::{application, endpoint},
    error::{Error, Result},
};

/// The information cached during the creation of a message. Includes a [`Vec`] of all endpoints
/// associated with the given application and organization ID.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateMessageApp {
    pub id: ApplicationId,
    pub uid: Option<ApplicationUid>,
    pub org_id: OrganizationId,
    // TODO: org_group_id
    pub rate_limit: Option<u16>,
    pub endpoints: Vec<CreateMessageEndpoint>,
    pub deleted: bool,
}

/// The information for each individual endpoint cached with the creation of a message.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateMessageEndpoint {
    pub id: EndpointId,
    pub url: String,
    pub key: EndpointSecret,
    pub old_signing_keys: Option<ExpiringSigningKeys>,
    pub event_types_ids: Option<EventTypeNameSet>,
    pub channels: Option<EventChannelSet>,
    pub rate_limit: Option<u16>,
    // Same type as the `DateTimeWithTimeZone from SeaORM used in the endpoint model
    pub first_failure_at: Option<DateTime<FixedOffset>>,
    pub headers: Option<EndpointHeaders>,
    pub disabled: bool,
    pub deleted: bool,
}

impl CreateMessageApp {
    /// Fetch all requisite information for creating a [`CreateMessageApp`] from the PostgreSQL
    /// database
    pub async fn fetch(
        db: &DatabaseTransaction,
        app: application::Model,
    ) -> Result<CreateMessageApp> {
        let endpoints = endpoint::Entity::secure_find(app.id.clone())
            .all(db)
            .await?
            .into_iter()
            .map(|db_val| {
                Ok(CreateMessageEndpoint {
                    id: db_val.id,
                    url: db_val.url,
                    key: db_val.key,
                    old_signing_keys: db_val.old_keys,
                    event_types_ids: db_val.event_types_ids,
                    channels: db_val.channels,
                    rate_limit: db_val
                        .rate_limit
                        .map(|v| v.try_into())
                        .transpose()
                        .map_err(|_| {
                            Error::Validation("Endpoint rate limit out of bounds".to_owned())
                        })?,
                    first_failure_at: db_val.first_failure_at,
                    headers: db_val.headers,
                    disabled: db_val.disabled,
                    deleted: db_val.deleted,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(CreateMessageApp {
            id: app.id,
            uid: app.uid,
            org_id: app.org_id,
            rate_limit: app
                .rate_limit
                .map(|v| v.try_into())
                .transpose()
                .map_err(|_| {
                    Error::Validation("Application rate limit out of bounds".to_owned())
                })?,
            endpoints,
            deleted: app.deleted,
        })
    }
}

kv_def!(AppEndpointKey, CreateMessageApp);
impl AppEndpointKey {
    // FIXME: Rewrite doc comment when AppEndpointValue members are known
    /// Returns a key for fetching all cached endpoints for a given organization and application.
    pub fn new(org: OrganizationId, app: ApplicationId) -> AppEndpointKey {
        AppEndpointKey(format!("{}_APP_v3_{}_{}", Self::PREFIX_CACHE, org, app))
    }
}
