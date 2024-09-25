mod cluster;

use std::time::Duration;

use bb8::{Pool, RunError};
use bb8_redis::RedisConnectionManager;
use redis::{FromRedisValue, RedisError, RedisResult};

pub use self::cluster::RedisClusterConnectionManager;
use crate::cfg::{CacheBackend, QueueBackend};

pub const REDIS_CONN_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone, Debug)]
pub enum RedisManager {
    Clustered(ClusteredRedisPool),
    ClusteredUnpooled(ClusteredRedisUnpooled),
    NonClustered(NonClusteredRedisPool),
    NonClusteredUnpooled(NonClusteredRedisUnpooled),
}

impl RedisManager {
    async fn new_pooled(dsn: &str, clustered: bool, max_conns: u16) -> Self {
        if clustered {
            let mgr = RedisClusterConnectionManager::new(dsn)
                .expect("Error initializing redis cluster client");
            let pool = bb8::Pool::builder()
                .max_size(max_conns.into())
                .build(mgr)
                .await
                .expect("Error initializing redis cluster connection pool");
            let pool = ClusteredRedisPool { pool };
            RedisManager::Clustered(pool)
        } else {
            let mgr = RedisConnectionManager::new(dsn).expect("Error initializing redis client");
            let pool = bb8::Pool::builder()
                .max_size(max_conns.into())
                .build(mgr)
                .await
                .expect("Error initializing redis connection pool");
            let pool = NonClusteredRedisPool { pool };
            RedisManager::NonClustered(pool)
        }
    }

    async fn new_unpooled(dsn: &str, clustered: bool) -> Self {
        if clustered {
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
        } else {
            let cli = redis::Client::open(dsn).expect("Error initializing redis unpooled client");
            let con = redis::aio::ConnectionManager::new_with_backoff_and_timeouts(
                cli,
                2,
                100,
                1,
                Duration::MAX,
                REDIS_CONN_TIMEOUT,
            )
            .await
            .expect("Failed to get redis-unpooled connection manager");
            RedisManager::NonClusteredUnpooled(NonClusteredRedisUnpooled { con })
        }
    }

    pub async fn from_cache_backend(cache_backend: &CacheBackend<'_>) -> Self {
        match cache_backend {
            CacheBackend::Redis(dsn) => Self::new_unpooled(dsn, false).await,
            CacheBackend::RedisCluster(dsn) => Self::new_unpooled(dsn, true).await,
            _ => panic!("Queue type not supported with redis"),
        }
    }

    pub async fn from_queue_backend(queue_backend: &QueueBackend<'_>, max_conns: u16) -> Self {
        match queue_backend {
            QueueBackend::Redis(dsn) => Self::new_pooled(dsn, false, max_conns).await,
            QueueBackend::RedisCluster(dsn) => Self::new_pooled(dsn, true, max_conns).await,
            _ => panic!("Queue type not supported with redis"),
        }
    }

    pub async fn get(&self) -> Result<RedisConnection<'_>, RunError<RedisError>> {
        match self {
            Self::Clustered(pool) => pool.get().await,
            Self::NonClustered(pool) => pool.get().await,
            Self::ClusteredUnpooled(pool) => pool.get().await,
            Self::NonClusteredUnpooled(pool) => pool.get().await,
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
        }
    }

    fn get_db(&self) -> i64 {
        match self {
            RedisConnection::Clustered(conn) => conn.con.get_db(),
            RedisConnection::NonClustered(conn) => conn.con.get_db(),
            RedisConnection::ClusteredUnpooled(conn) => conn.con.get_db(),
            RedisConnection::NonClusteredUnpooled(conn) => conn.con.get_db(),
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

connection!(
    NonClusteredUnpooledConnection,
    redis::aio::ConnectionManager
);

connection!(
    ClusteredUnpooledConnection,
    redis::cluster_async::ClusterConnection
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
