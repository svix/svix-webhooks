use redis::{
    sentinel::{SentinelClient, SentinelNodeConnectionInfo, SentinelServerType},
    ErrorKind, IntoConnectionInfo, RedisError,
};
use std::sync::Mutex;

struct LockedSentinelClient(pub(crate) Mutex<SentinelClient>);

/// ConnectionManager that implements `bb8::ManageConnection` and supports
/// asynchronous Sentinel connections via `redis::sentinel::SentinelClient`
pub struct RedisSentinelConnectionManager {
    client: LockedSentinelClient,
}

impl RedisSentinelConnectionManager {
    pub fn new<T: IntoConnectionInfo>(
        info: Vec<T>,
        service_name: String,
        node_connection_info: Option<SentinelNodeConnectionInfo>,
    ) -> Result<RedisSentinelConnectionManager, RedisError> {
        Ok(RedisSentinelConnectionManager {
            client: LockedSentinelClient(Mutex::new(SentinelClient::build(
                info,
                service_name,
                node_connection_info,
                SentinelServerType::Master,
            )?)),
        })
    }
}

impl bb8::ManageConnection for RedisSentinelConnectionManager {
    type Connection = redis::aio::MultiplexedConnection;
    type Error = RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.0.lock().unwrap().get_async_connection().unwrap()
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(conn).await?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((ErrorKind::ResponseError, "ping request").into()),
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
