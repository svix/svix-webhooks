use std::convert::{TryFrom, TryInto};

use chrono::{DateTime, FixedOffset};
use sea_orm::DatabaseTransaction;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        cache::{kv_def, CacheKey, CacheValue},
        types::{
            ApplicationId, ApplicationUid, EndpointHeaders, EndpointId, EndpointSecret,
            EventChannelSet, EventTypeNameSet, ExpiringSigningKeys, OrganizationId,
        },
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
    pub rate_limit: Option<u16>,
    pub endpoints: Vec<CreateMessageEndpoint>,
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
            .map(TryInto::try_into)
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

impl TryFrom<endpoint::Model> for CreateMessageEndpoint {
    type Error = Error;

    fn try_from(m: endpoint::Model) -> Result<CreateMessageEndpoint> {
        Ok(CreateMessageEndpoint {
            id: m.id,
            url: m.url,
            key: m.key,
            old_signing_keys: m.old_keys,
            event_types_ids: m.event_types_ids,
            channels: m.channels,
            rate_limit: m
                .rate_limit
                .map(|v| v.try_into())
                .transpose()
                .map_err(|_| Error::Validation("Endpoint rate limit out of bounds".to_owned()))?,
            first_failure_at: m.first_failure_at,
            headers: m.headers,
            disabled: m.disabled,
            deleted: m.deleted,
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
