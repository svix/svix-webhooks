use std::sync::{Arc, RwLock};

use redis::{
    AsyncConnectionConfig, ErrorKind, IntoConnectionInfo, RedisError, Value,
    aio::{ConnectionLike, MultiplexedConnection},
    sentinel::{SentinelClient, SentinelNodeConnectionInfo, SentinelServerType},
};
use tokio::sync::Mutex;

use super::REDIS_CONN_TIMEOUT;

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
        self.client.0.lock().await.get_async_connection().await
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

/// A single multiplexed connection to the master of a Sentinel-monitored
/// deployment, shared by all callers.
///
/// The master is resolved through Sentinel when the first connection is
/// requested, and again after the current connection has failed in a way that
/// suggests it is broken or no longer connected to the master (see
/// [`should_reconnect`]). Like `redis::aio::ConnectionManager`, a command that
/// runs into such a failure is not retried, only later ones get a new
/// connection.
#[derive(Clone)]
pub struct RedisSentinelUnpooledManager(Arc<Inner>);

struct Inner {
    client: Mutex<SentinelClient>,
    current: RwLock<Current>,
}

#[derive(Clone, Default)]
struct Current {
    /// Incremented for every new connection, so that a connection that failed
    /// for many callers at once is only discarded once.
    generation: u64,
    conn: Option<MultiplexedConnection>,
}

impl RedisSentinelUnpooledManager {
    pub fn new(client: SentinelClient) -> Self {
        Self(Arc::new(Inner {
            client: Mutex::new(client),
            current: RwLock::default(),
        }))
    }

    pub async fn get(&self) -> Result<RedisSentinelUnpooledConnection, RedisError> {
        if let Some(conn) = self.current() {
            return Ok(conn);
        }

        let mut client = self.0.client.lock().await;
        // Another task may have reconnected while we were waiting for the lock.
        if let Some(conn) = self.current() {
            return Ok(conn);
        }

        let conn = client
            .get_async_connection_with_config(
                &AsyncConnectionConfig::new().set_response_timeout(REDIS_CONN_TIMEOUT),
            )
            .await?;

        let mut current = self.0.current.write().unwrap();
        current.generation += 1;
        current.conn = Some(conn.clone());

        Ok(RedisSentinelUnpooledConnection {
            manager: self.clone(),
            generation: current.generation,
            conn,
        })
    }

    fn current(&self) -> Option<RedisSentinelUnpooledConnection> {
        let Current { generation, conn } = self.0.current.read().unwrap().clone();
        Some(RedisSentinelUnpooledConnection {
            manager: self.clone(),
            generation,
            conn: conn?,
        })
    }

    /// Discard the connection of the given generation, unless it was already
    /// replaced, so that the next call to [`Self::get`] asks Sentinel for the
    /// master again.
    fn discard(&self, generation: u64) {
        let mut current = self.0.current.write().unwrap();
        if current.generation == generation {
            current.conn = None;
        }
    }
}

/// Whether an error means that the connection should not be used any longer.
///
/// That is the case for I/O errors (which includes a closed connection and
/// response timeouts) and for errors that are only returned by replicas: after
/// a failover, the node we are connected to may still be reachable, but no
/// longer be the master.
fn should_reconnect(e: &RedisError) -> bool {
    e.is_io_error()
        || e.is_unrecoverable_error()
        || matches!(e.kind(), ErrorKind::ReadOnly | ErrorKind::MasterDown)
}

/// Same as [`should_reconnect`], for a reply. The connection returns errors
/// sent by the server as part of the reply, it is the caller that turns them
/// into a [`RedisError`] afterwards.
fn is_reconnect_reply(reply: &Value) -> bool {
    matches!(reply, Value::ServerError(e) if should_reconnect(&e.clone().into()))
}

/// A handle to the connection shared through a
/// [`RedisSentinelUnpooledManager`].
pub struct RedisSentinelUnpooledConnection {
    manager: RedisSentinelUnpooledManager,
    generation: u64,
    conn: MultiplexedConnection,
}

impl RedisSentinelUnpooledConnection {
    /// Discard the shared connection if the outcome of a request shows that
    /// it should not be used any longer.
    fn check<T>(
        &self,
        res: Result<T, RedisError>,
        replies: impl FnOnce(&T) -> &[Value],
    ) -> Result<T, RedisError> {
        let reconnect = match &res {
            Ok(v) => replies(v).iter().any(is_reconnect_reply),
            Err(e) => should_reconnect(e),
        };
        if reconnect {
            self.manager.discard(self.generation);
        }
        res
    }
}

impl ConnectionLike for RedisSentinelUnpooledConnection {
    fn req_packed_command<'a>(&'a mut self, cmd: &'a redis::Cmd) -> redis::RedisFuture<'a, Value> {
        Box::pin(async move {
            let res = self.conn.req_packed_command(cmd).await;
            self.check(res, std::slice::from_ref)
        })
    }

    fn req_packed_commands<'a>(
        &'a mut self,
        cmd: &'a redis::Pipeline,
        offset: usize,
        count: usize,
    ) -> redis::RedisFuture<'a, Vec<Value>> {
        Box::pin(async move {
            let res = self.conn.req_packed_commands(cmd, offset, count).await;
            self.check(res, Vec::as_slice)
        })
    }

    fn get_db(&self) -> i64 {
        self.conn.get_db()
    }
}

#[cfg(test)]
mod tests {
    use redis::{ErrorKind, RedisError, parse_redis_value};

    use super::{is_reconnect_reply, should_reconnect};

    #[test]
    fn test_replica_errors_cause_reconnect() {
        for (reply, expected) in [
            (
                "-READONLY You can't write against a read only replica.\r\n",
                true,
            ),
            (
                "-MASTERDOWN Link with MASTER is down and replica-serve-stale-data is set to 'no'.\r\n",
                true,
            ),
            ("-ERR unknown command\r\n", false),
            ("+OK\r\n", false),
        ] {
            let reply = parse_redis_value(reply.as_bytes()).unwrap();
            assert_eq!(is_reconnect_reply(&reply), expected, "{reply:?}");
        }
    }

    #[test]
    fn test_io_errors_cause_reconnect() {
        let closed = std::io::Error::from(std::io::ErrorKind::BrokenPipe);
        assert!(should_reconnect(&RedisError::from(closed)));
        let timeout = std::io::Error::from(std::io::ErrorKind::TimedOut);
        assert!(should_reconnect(&RedisError::from(timeout)));
        assert!(!should_reconnect(&RedisError::from((
            ErrorKind::TypeError,
            "not compatible"
        ))));
    }
}
