mod cluster;
mod sentinel;

use std::{sync::Arc, time::Duration};

use bb8::{Pool, RunError};
use bb8_redis::RedisConnectionManager;
use redis::{
    aio::ConnectionManagerConfig, sentinel::SentinelNodeConnectionInfo, AsyncConnectionConfig,
    FromRedisValue, ProtocolVersion, RedisConnectionInfo, RedisError, RedisResult, TlsMode,
};
use sentinel::RedisSentinelConnectionManager;
use tokio::sync::Mutex;

pub use self::cluster::RedisClusterConnectionManager;
use crate::cfg::{CacheBackend, QueueBackend, SentinelConfig};

pub const REDIS_CONN_TIMEOUT: Duration = Duration::from_secs(2);

pub enum RedisVariant<'a> {
    Clustered,
    NonClustered,
    Sentinel(&'a SentinelConfig),
}

#[derive(Clone, Debug)]
pub enum RedisManager {
    Clustered(ClusteredRedisPool),
    ClusteredUnpooled(ClusteredRedisUnpooled),
    NonClustered(NonClusteredRedisPool),
    NonClusteredUnpooled(NonClusteredRedisUnpooled),
    Sentinel(SentinelPooled),
    SentinelUnpooled(SentinelUnpooled),
}

impl RedisManager {
    async fn new_pooled(dsn: &str, variant: RedisVariant<'_>, max_conns: u16) -> Self {
        match variant {
            RedisVariant::Clustered => {
                let mgr = RedisClusterConnectionManager::new(dsn)
                    .expect("Error initializing redis cluster client");
                let pool = bb8::Pool::builder()
                    .max_size(max_conns.into())
                    .build(mgr)
                    .await
                    .expect("Error initializing redis cluster connection pool");
                let pool = ClusteredRedisPool { pool };
                RedisManager::Clustered(pool)
            }
            RedisVariant::NonClustered => {
                let mgr =
                    RedisConnectionManager::new(dsn).expect("Error initializing redis client");
                let pool = bb8::Pool::builder()
                    .max_size(max_conns.into())
                    .build(mgr)
                    .await
                    .expect("Error initializing redis connection pool");
                let pool = NonClusteredRedisPool { pool };
                RedisManager::NonClustered(pool)
            }
            RedisVariant::Sentinel(cfg) => {
                let tls_mode = cfg.redis_tls_mode_secure.then_some(TlsMode::Secure);
                let protocol = if cfg.redis_use_resp3 {
                    ProtocolVersion::RESP3
                } else {
                    ProtocolVersion::default()
                };
                let mgr = RedisSentinelConnectionManager::new(
                    vec![dsn],
                    cfg.service_name.clone(),
                    Some(SentinelNodeConnectionInfo {
                        tls_mode,
                        redis_connection_info: Some(RedisConnectionInfo {
                            db: cfg.redis_db.unwrap_or(0),
                            username: cfg.redis_username.clone(),
                            password: cfg.redis_password.clone(),
                            protocol,
                        }),
                    }),
                )
                .expect("Error initializing RedisSentinelConnectionManager");
                let pool = bb8::Pool::builder()
                    .max_size(max_conns.into())
                    .build(mgr)
                    .await
                    .expect("Error initializing redis connection pool");

                let pool = SentinelPooled { pool };
                RedisManager::Sentinel(pool)
            }
        }
    }

    async fn new_unpooled(dsn: &str, variant: RedisVariant<'_>) -> Self {
        match variant {
            RedisVariant::Clustered => {
                let cli = redis::cluster::ClusterClient::builder(vec![dsn])
                    .retries(1)
                    .connection_timeout(REDIS_CONN_TIMEOUT)
                    .build()
                    .expect("Error initializing redis-unpooled cluster client");
                let con = cli
                    .get_async_connection()
                    .await
                    .expect("Failed to get redis-cluster-unpooled connection");
                RedisManager::ClusteredUnpooled(ClusteredRedisUnpooled { con })
            }
            RedisVariant::NonClustered => {
                let cli =
                    redis::Client::open(dsn).expect("Error initializing redis unpooled client");
                let con = redis::aio::ConnectionManager::new_with_config(
                    cli,
                    ConnectionManagerConfig::new()
                        .set_number_of_retries(1)
                        .set_connection_timeout(REDIS_CONN_TIMEOUT),
                )
                .await
                .expect("Failed to get redis-unpooled connection manager");
                RedisManager::NonClusteredUnpooled(NonClusteredRedisUnpooled { con })
            }
            RedisVariant::Sentinel(cfg) => {
                let tls_mode = cfg.redis_tls_mode_secure.then_some(TlsMode::Secure);
                let protocol = if cfg.redis_use_resp3 {
                    ProtocolVersion::RESP3
                } else {
                    ProtocolVersion::default()
                };
                let cli = redis::sentinel::SentinelClient::build(
                    vec![dsn],
                    cfg.service_name.clone(),
                    Some(SentinelNodeConnectionInfo {
                        tls_mode,
                        redis_connection_info: Some(RedisConnectionInfo {
                            db: cfg.redis_db.unwrap_or(0),
                            username: cfg.redis_username.clone(),
                            password: cfg.redis_password.clone(),
                            protocol,
                        }),
                    }),
                    redis::sentinel::SentinelServerType::Master,
                )
                .expect("Failed to build sentinel client");

                RedisManager::SentinelUnpooled(SentinelUnpooled {
                    pool: Arc::new(Mutex::new(cli)),
                })
            }
        }
    }

    pub async fn from_cache_backend(cache_backend: &CacheBackend<'_>) -> Self {
        match cache_backend {
            CacheBackend::Redis(dsn) => Self::new_unpooled(dsn, RedisVariant::NonClustered).await,
            CacheBackend::RedisCluster(dsn) => {
                Self::new_unpooled(dsn, RedisVariant::Clustered).await
            }
            CacheBackend::RedisSentinel(dsn, cfg) => {
                Self::new_unpooled(dsn, RedisVariant::Sentinel(cfg)).await
            }
            _ => panic!("Queue type not supported with redis"),
        }
    }

    pub async fn from_queue_backend(queue_backend: &QueueBackend<'_>, max_conns: u16) -> Self {
        match queue_backend {
            QueueBackend::Redis(dsn) => {
                Self::new_pooled(dsn, RedisVariant::NonClustered, max_conns).await
            }
            QueueBackend::RedisCluster(dsn) => {
                Self::new_pooled(dsn, RedisVariant::Clustered, max_conns).await
            }
            QueueBackend::RedisSentinel(dsn, cfg) => {
                Self::new_pooled(dsn, RedisVariant::Sentinel(cfg), max_conns).await
            }
            _ => panic!("Queue type not supported with redis"),
        }
    }

    pub async fn get(&self) -> Result<RedisConnection<'_>, RunError<RedisError>> {
        match self {
            Self::Clustered(pool) => pool.get().await,
            Self::NonClustered(pool) => pool.get().await,
            Self::ClusteredUnpooled(pool) => pool.get().await,
            Self::NonClusteredUnpooled(pool) => pool.get().await,
            Self::Sentinel(pool) => pool.get().await,
            Self::SentinelUnpooled(pool) => pool.get().await,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClusteredRedisPool {
    pool: Pool<RedisClusterConnectionManager>,
}

impl ClusteredRedisPool {
    pub async fn get(&self) -> Result<RedisConnection<'_>, RunError<RedisError>> {
        let con = ClusteredPooledConnection {
            con: self.pool.get().await?,
        };
        Ok(RedisConnection::Clustered(con))
    }
}

#[derive(Clone)]
pub struct ClusteredRedisUnpooled {
    con: redis::cluster_async::ClusterConnection,
}

impl ClusteredRedisUnpooled {
    pub async fn get(&self) -> Result<RedisConnection<'_>, RunError<RedisError>> {
        Ok(RedisConnection::ClusteredUnpooled(
            ClusteredUnpooledConnection {
                con: self.con.clone(),
            },
        ))
    }
}

impl std::fmt::Debug for ClusteredRedisUnpooled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClusteredRedisUnpooled").finish()
    }
}

#[derive(Clone)]
pub struct NonClusteredRedisUnpooled {
    con: redis::aio::ConnectionManager,
}

impl NonClusteredRedisUnpooled {
    pub async fn get(&self) -> Result<RedisConnection<'_>, RunError<RedisError>> {
        Ok(RedisConnection::NonClusteredUnpooled(
            NonClusteredUnpooledConnection {
                con: self.con.clone(),
            },
        ))
    }
}

impl std::fmt::Debug for NonClusteredRedisUnpooled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NonClusteredRedisUnpooled").finish()
    }
}

#[derive(Clone)]
pub struct SentinelPooled {
    pool: Pool<crate::redis::sentinel::RedisSentinelConnectionManager>,
}

impl SentinelPooled {
    pub async fn get(&self) -> Result<RedisConnection<'_>, RunError<RedisError>> {
        Ok(RedisConnection::SentinelPooled(SentinelPooledConnection {
            con: self.pool.get().await?,
        }))
    }
}

impl std::fmt::Debug for SentinelPooled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SentinelPooled").finish()
    }
}

#[derive(Clone)]
pub struct SentinelUnpooled {
    pool: Arc<Mutex<redis::sentinel::SentinelClient>>,
}

impl SentinelUnpooled {
    pub async fn get(&self) -> Result<RedisConnection<'_>, RunError<RedisError>> {
        let mut pool = (*self.pool).lock().await;
        let con = pool
            .get_async_connection_with_config(
                &AsyncConnectionConfig::new().set_response_timeout(REDIS_CONN_TIMEOUT),
            )
            .await?;
        Ok(RedisConnection::SentinelUnpooled(
            SentinelUnpooledConnection { con },
        ))
    }
}

impl std::fmt::Debug for SentinelUnpooled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SentinelUnpooled").finish()
    }
}

#[derive(Clone, Debug)]
pub struct NonClusteredRedisPool {
    pool: Pool<RedisConnectionManager>,
}

impl NonClusteredRedisPool {
    pub async fn get(&self) -> Result<RedisConnection<'_>, RunError<RedisError>> {
        let con = self.pool.get().await?;
        let con = NonClusteredPooledConnection { con };
        Ok(RedisConnection::NonClustered(con))
    }
}

pub enum RedisConnection<'a> {
    Clustered(ClusteredPooledConnection<'a>),
    ClusteredUnpooled(ClusteredUnpooledConnection),
    NonClustered(NonClusteredPooledConnection<'a>),
    NonClusteredUnpooled(NonClusteredUnpooledConnection),
    SentinelPooled(SentinelPooledConnection<'a>),
    SentinelUnpooled(SentinelUnpooledConnection),
}

impl RedisConnection<'_> {
    pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        cmd.query_async(self).await
    }

    pub async fn query_async_pipeline<T: FromRedisValue>(
        &mut self,
        pipe: redis::Pipeline,
    ) -> RedisResult<T> {
        pipe.query_async(self).await
    }
}

impl redis::aio::ConnectionLike for RedisConnection<'_> {
    fn req_packed_command<'a>(
        &'a mut self,
        cmd: &'a redis::Cmd,
    ) -> redis::RedisFuture<'a, redis::Value> {
        match self {
            RedisConnection::Clustered(conn) => conn.con.req_packed_command(cmd),
            RedisConnection::NonClustered(conn) => conn.con.req_packed_command(cmd),
            RedisConnection::ClusteredUnpooled(conn) => conn.con.req_packed_command(cmd),
            RedisConnection::NonClusteredUnpooled(conn) => conn.con.req_packed_command(cmd),
            RedisConnection::SentinelPooled(conn) => conn.con.req_packed_command(cmd),
            RedisConnection::SentinelUnpooled(conn) => conn.con.req_packed_command(cmd),
        }
    }

    fn req_packed_commands<'a>(
        &'a mut self,
        cmd: &'a redis::Pipeline,
        offset: usize,
        count: usize,
    ) -> redis::RedisFuture<'a, Vec<redis::Value>> {
        match self {
            RedisConnection::Clustered(conn) => conn.con.req_packed_commands(cmd, offset, count),
            RedisConnection::NonClustered(conn) => conn.con.req_packed_commands(cmd, offset, count),
            RedisConnection::ClusteredUnpooled(conn) => {
                conn.con.req_packed_commands(cmd, offset, count)
            }
            RedisConnection::NonClusteredUnpooled(conn) => {
                conn.con.req_packed_commands(cmd, offset, count)
            }
            RedisConnection::SentinelPooled(conn) => {
                conn.con.req_packed_commands(cmd, offset, count)
            }
            RedisConnection::SentinelUnpooled(conn) => {
                conn.con.req_packed_commands(cmd, offset, count)
            }
        }
    }

    fn get_db(&self) -> i64 {
        match self {
            RedisConnection::Clustered(conn) => conn.con.get_db(),
            RedisConnection::NonClustered(conn) => conn.con.get_db(),
            RedisConnection::ClusteredUnpooled(conn) => conn.con.get_db(),
            RedisConnection::NonClusteredUnpooled(conn) => conn.con.get_db(),
            RedisConnection::SentinelPooled(conn) => conn.con.get_db(),
            RedisConnection::SentinelUnpooled(conn) => conn.con.get_db(),
        }
    }
}

macro_rules! pooled_connection {
    (
        $(
            $struct_name:ident,
            $con_type:ty
        ),*
    ) => {
        $(
            pub struct $struct_name<'a> {
                con: $con_type,
            }

            impl<'a> $struct_name<'a> {
                pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
                    cmd.query_async(&mut *self.con).await
                }

                pub async fn query_async_pipeline<T: FromRedisValue>(&mut self, pipe: redis::Pipeline) -> RedisResult<T> {
                    pipe.query_async(&mut *self.con).await
                }
            }
        )*
    }
}

macro_rules! connection {
    (
        $(
            $struct_name:ident,
            $con_type:ty
        ),*
    ) => {
        $(
            pub struct $struct_name {
                con: $con_type,
            }

            impl $struct_name {
                pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
                    cmd.query_async(&mut self.con).await
                }

                pub async fn query_async_pipeline<T: FromRedisValue>(&mut self, pipe: redis::Pipeline) -> RedisResult<T> {
                    pipe.query_async(&mut self.con).await
                }
            }
        )*
    }
}

pooled_connection!(
    NonClusteredPooledConnection,
    bb8::PooledConnection<'a, RedisConnectionManager>
);

pooled_connection!(
    ClusteredPooledConnection,
    bb8::PooledConnection<'a, RedisClusterConnectionManager>
);

pooled_connection!(
    SentinelPooledConnection,
    bb8::PooledConnection<'a, RedisSentinelConnectionManager>
);

connection!(
    NonClusteredUnpooledConnection,
    redis::aio::ConnectionManager
);

connection!(
    ClusteredUnpooledConnection,
    redis::cluster_async::ClusterConnection
);

connection!(
    SentinelUnpooledConnection,
    redis::aio::MultiplexedConnection
);

#[cfg(test)]
mod tests {
    use redis::AsyncCommands;

    use super::RedisManager;

    // Ensure basic set/get works -- should test sharding as well:
    #[tokio::test]
    // run with `cargo test -- --ignored redis` only when redis is up and configured
    #[ignore]
    async fn test_set_read_random_keys() {
        dotenvy::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let mgr = RedisManager::from_cache_backend(&cfg.cache_backend()).await;
        let mut conn = mgr.get().await.unwrap();

        for (val, key) in "abcdefghijklmnopqrstuvwxyz".chars().enumerate() {
            let key = key.to_string();
            let _: () = conn.set(key.clone(), val).await.unwrap();
            assert_eq!(conn.get::<_, usize>(&key).await.unwrap(), val);
        }
    }
}
