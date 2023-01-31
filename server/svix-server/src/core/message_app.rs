use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
    time::Duration,
};

use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        cache::{kv_def, Cache, CacheBehavior, CacheKey, CacheValue},
        types::{
            ApplicationId, ApplicationUid, EndpointHeaders, EndpointId, EndpointSecretInternal,
            EventChannelSet, EventTypeNameSet, ExpiringSigningKeys, MessageAttemptTriggerType,
            OrganizationId,
        },
    },
    ctx,
    db::models::{application, endpoint},
    err_validation,
    error::{Error, Result},
};

use super::types::EventTypeName;

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
    async fn fetch_from_pg_by_model(
        db: &DatabaseTransaction,
        app: application::Model,
    ) -> Result<CreateMessageApp> {
        let endpoints = ctx!(endpoint::Entity::secure_find(app.id.clone()).all(db).await)?
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
                .map_err(|_| err_validation!("Application rate limit out of bounds"))?,
            endpoints,
            deleted: app.deleted,
        })
    }

    /// Fetches all information for creating a [`CreateMessageApp`] from the Redis cache if it
    /// exists or from PostgreSQL otherwise. If the RedisCache is Some, but does not contain the
    /// requisite information, fetch it from PostgreSQL and insert the data into the cache.
    pub async fn layered_fetch(
        cache: &Cache,
        pg: &DatabaseConnection,
        app: Option<application::Model>,
        org_id: OrganizationId,
        app_id: ApplicationId,
        ttl: Duration,
    ) -> Result<Option<CreateMessageApp>> {
        let cache_key = AppEndpointKey::new(&org_id, &app_id);

        // First check Redis
        if let Ok(Some(cma)) = cache.get(&cache_key).await {
            return Ok(Some(cma));
        }

        // Then check PostgreSQL
        let db = ctx!(pg.begin().await)?;
        // Fetch the [`application::Model`] either given or from the ID
        let app = if let Some(app) = app {
            app
        } else if let Some(app) = ctx!(
            application::Entity::secure_find_by_id(org_id, app_id)
                .one(&db)
                .await
        )? {
            app
        } else {
            return Ok(None);
        };

        // Fetch the actual [`CreateMessageApp`]
        let out = Self::fetch_from_pg_by_model(&db, app).await?;

        // Insert it into Redis
        let _ = cache.set(&cache_key, &out, ttl).await;

        Ok(Some(out))
    }

    pub fn filtered_endpoints(
        &self,
        trigger_type: MessageAttemptTriggerType,
        event_type: &EventTypeName,
        channels: Option<&EventChannelSet>,
    ) -> Vec<CreateMessageEndpoint> {
        self
        .endpoints
        .iter()
        .filter(|endpoint| {
            return
            // No disabled or deleted endpoints ever
               !endpoint.disabled && !endpoint.deleted &&
            (
                // Manual attempt types go through regardless
                trigger_type == MessageAttemptTriggerType::Manual
                || (
                        // If an endpoint has event types and it matches ours, or has no event types
                        endpoint
                        .event_types_ids
                        .as_ref()
                        .map(|x| x.0.contains(event_type))
                        .unwrap_or(true)
                    &&
                        // If an endpoint has no channels accept all messages, otherwise only if their channels overlap.
                        // A message with no channels doesn't match an endpoint with channels.
                        endpoint
                        .channels
                        .as_ref()
                        .map(|x| !x.0.is_disjoint(channels.map(|x| &x.0).unwrap_or(&HashSet::new())))
                        .unwrap_or(true)
            ))})
        .cloned()
        .collect()
    }
}

/// The information for each individual endpoint cached with the creation of a message.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateMessageEndpoint {
    pub id: EndpointId,
    pub url: String,
    pub key: EndpointSecretInternal,
    pub event_types_ids: Option<EventTypeNameSet>,
    pub channels: Option<EventChannelSet>,
    pub rate_limit: Option<u16>,
    // Same type as the `DateTimeWithTimeZone from SeaORM used in the endpoint model
    pub first_failure_at: Option<DateTime<FixedOffset>>,
    pub headers: Option<EndpointHeaders>,
    pub disabled: bool,
    pub deleted: bool,
    // outside of this module, valid_signing_keys should be used instead
    old_signing_keys: Option<ExpiringSigningKeys>,
}

impl CreateMessageEndpoint {
    pub fn valid_signing_keys(&self) -> Vec<&EndpointSecretInternal> {
        match self.old_signing_keys {
            Some(ref old_keys) => std::iter::once(&self.key)
                .chain(
                    old_keys
                        .0
                        .iter()
                        .filter(|x| x.expiration > Utc::now())
                        .map(|x| &x.key),
                )
                .collect(),
            None => vec![&self.key],
        }
    }
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
                .map_err(|_| err_validation!("Endpoint rate limit out of bounds"))?,
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
    pub fn new(org: &OrganizationId, app: &ApplicationId) -> AppEndpointKey {
        AppEndpointKey(format!("{}_APP_v3_{}_{}", Self::PREFIX_CACHE, org, app))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cryptography::Encryption;
    use crate::core::types::{EndpointSecret, ExpiringSigningKey};

    #[test]
    fn test_valid_signing_keys() {
        let key = EndpointSecretInternal::from_endpoint_secret(
            EndpointSecret::Symmetric(base64::decode("MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw").unwrap()),
            &Encryption::new_noop(),
        )
        .unwrap();

        let unexpired_old_key = ExpiringSigningKey {
            key: key.clone(),
            expiration: Utc::now()
                + chrono::Duration::hours(ExpiringSigningKeys::OLD_KEY_EXPIRY_HOURS),
        };
        let expired_old_key = ExpiringSigningKey {
            key: key.clone(),
            expiration: Utc::now()
                - chrono::Duration::hours(ExpiringSigningKeys::OLD_KEY_EXPIRY_HOURS),
        };
        let old_signing_keys = Some(ExpiringSigningKeys(vec![
            unexpired_old_key,
            expired_old_key,
        ]));

        let cme = CreateMessageEndpoint {
            id: EndpointId::from("Test".to_string()),
            url: "".to_string(),
            key,
            old_signing_keys,
            event_types_ids: None,
            channels: None,
            rate_limit: None,
            first_failure_at: None,
            headers: None,
            disabled: false,
            deleted: false,
        };

        let keys = cme.valid_signing_keys();

        assert_eq!(keys.len(), 2);
    }
}
