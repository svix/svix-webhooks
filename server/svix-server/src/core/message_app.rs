use std::{
    convert::{TryFrom, TryInto},
    time::Duration,
};

use bb8::ManageConnection;
use chrono::{DateTime, FixedOffset};
use redis::aio::ConnectionLike;
use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        cache::{kv_def, CacheKey, CacheValue, RedisCache},
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
    async fn fetch_from_pg_by_model(
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

    /// Fetches all information for creating a [`CreateMessageApp`] from the Redis cache if it
    /// exists or from PostgreSQL otherwise. If the RedisCache is Some, but does not contain the
    /// requisite information, fetch it from PostgreSQL and insert the data into the cache.
    pub async fn layered_fetch<M, N>(
        redis: Option<&RedisCache<M>>,
        redis_cluster: Option<&RedisCache<N>>,
        pg: &DatabaseConnection,
        app: Option<application::Model>,
        app_id: ApplicationId,
        org_id: OrganizationId,
        ttl: Duration,
    ) -> Result<Option<CreateMessageApp>>
    where
        M: ManageConnection + Clone,
        M::Connection: ConnectionLike,
        N: ManageConnection + Clone,
        N::Connection: ConnectionLike,
    {
        let cache_key = AppEndpointKey::new(org_id.clone(), app_id.clone());

        // First check Redis
        if let Some(redis) = redis {
            if let Ok(Some(cma)) = redis.get(&cache_key).await {
                return Ok(Some(cma));
            }
        } else if let Some(redis) = redis_cluster {
            if let Ok(Some(cma)) = redis.get(&cache_key).await {
                return Ok(Some(cma));
            }
        }

        // Then check PostgreSQL
        let db = pg.begin().await?;
        // Fetch the [`application::Model`] either given or from the ID
        let app = if let Some(app) = app {
            app
        } else if let Some(app) = application::Entity::secure_find_by_id(org_id, app_id)
            .one(&db)
            .await?
        {
            app
        } else {
            return Ok(None);
        };

        // Fetch the actual [`CreateMessageApp`]
        let out = Self::fetch_from_pg_by_model(&db, app).await?;

        // Insert it into Redis
        if let Some(redis) = redis {
            let _ = redis.set(&cache_key, &out, ttl).await;
        }

        Ok(Some(out))
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
