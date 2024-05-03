use axum::async_trait;
use redis::{
    cluster::{ClusterClient, ClusterClientBuilder},
    cluster_routing::{MultipleNodeRoutingInfo, ResponsePolicy, RoutingInfo},
    ErrorKind, FromRedisValue, IntoConnectionInfo, RedisError,
};

/// ConnectionManager that implements `bb8::ManageConnection` and supports
/// asynchronous clustered connections via `redis_cluster_async::Connection`
#[derive(Clone)]
pub struct RedisClusterConnectionManager {
    client: ClusterClient,
}

impl RedisClusterConnectionManager {
    pub fn new<T: IntoConnectionInfo>(
        info: T,
    ) -> Result<RedisClusterConnectionManager, RedisError> {
        Ok(RedisClusterConnectionManager {
            client: ClusterClientBuilder::new(vec![info]).retries(0).build()?,
        })
    }
}

#[async_trait]
impl bb8::ManageConnection for RedisClusterConnectionManager {
    type Connection = redis::cluster_async::ClusterConnection;
    type Error = RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_async_connection().await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong = conn
            .route_command(
                &redis::cmd("PING"),
                RoutingInfo::MultiNode((
                    MultipleNodeRoutingInfo::AllMasters,
                    Some(ResponsePolicy::OneSucceeded),
                )),
            )
            .await
            .and_then(|v| String::from_redis_value(&v))?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((ErrorKind::ResponseError, "ping request").into()),
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
