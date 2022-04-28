mod cluster;

use bb8::{Pool, PooledConnection, RunError};
use bb8_redis::RedisConnectionManager;
pub use cluster::RedisClusterConnectionManager;

use axum::async_trait;
use redis::{FromRedisValue, RedisError, RedisResult};

#[derive(Clone, Debug)]
pub enum SvixRedisPool {
    Clustered(SvixClusteredRedisPool),
    NonClustered(SvixNonClusteredRedisPool),
}

#[derive(Clone, Debug)]
pub struct SvixClusteredRedisPool {
    pool: Pool<RedisClusterConnectionManager>,
}

#[derive(Clone, Debug)]
pub struct SvixNonClusteredRedisPool {
    pool: Pool<RedisConnectionManager>,
}

pub enum SvixPooledConnection<'a> {
    Clustered(SvixClusteredPooledConnection<'a>),
    NonClustered(SvixNonClusteredPooledConnection<'a>),
}

impl<'a> SvixPooledConnection<'a> {
    pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        match self {
            Self::Clustered(pooled_con) => pooled_con.query_async(cmd).await,
            Self::NonClustered(pooled_con) => pooled_con.query_async(cmd).await,
        }
    }
}

pub struct SvixNonClusteredPooledConnection<'a> {
    con: PooledConnection<'a, RedisConnectionManager>,
}

impl<'a> SvixNonClusteredPooledConnection<'a> {
    pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        cmd.query_async(&mut *self.con).await
    }
}

pub struct SvixClusteredPooledConnection<'a> {
    con: PooledConnection<'a, RedisClusterConnectionManager>,
}

impl<'a> SvixClusteredPooledConnection<'a> {
    pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        cmd.query_async(&mut *self.con).await
    }
}

#[async_trait]
pub trait PoolLike {
    async fn get(&self) -> Result<SvixPooledConnection, RunError<RedisError>>;
}

#[async_trait]
impl PoolLike for SvixRedisPool {
    async fn get(&self) -> Result<SvixPooledConnection, RunError<RedisError>> {
        match self {
            Self::Clustered(pool) => pool.get().await,
            Self::NonClustered(pool) => pool.get().await,
        }
    }
}

#[async_trait]
impl PoolLike for SvixNonClusteredRedisPool {
    async fn get(&self) -> Result<SvixPooledConnection, RunError<RedisError>> {
        let con = self.pool.get().await?;
        let con = SvixNonClusteredPooledConnection { con };
        Ok(SvixPooledConnection::NonClustered(con))
    }
}

#[async_trait]
impl PoolLike for SvixClusteredRedisPool {
    async fn get(&self) -> Result<SvixPooledConnection, RunError<RedisError>> {
        let con = SvixClusteredPooledConnection {
            con: self.pool.get().await?,
        };
        Ok(SvixPooledConnection::Clustered(con))
    }
}

pub async fn create_redis_pool(redis_dsn: &str, clustered: bool) -> SvixRedisPool {
    if clustered {
        let mgr = RedisClusterConnectionManager::new(redis_dsn)
            .expect("Error initializing redis cluster client");
        let pool = bb8::Pool::builder()
            .build(mgr)
            .await
            .expect("Error initializing redis cluster connection pool");
        let pool = SvixClusteredRedisPool { pool };
        SvixRedisPool::Clustered(pool)
    } else {
        let mgr = RedisConnectionManager::new(redis_dsn).expect("Error intializing redis client");
        let pool = bb8::Pool::builder()
            .build(mgr)
            .await
            .expect("Error initializing redis connection pool");
        let pool = SvixNonClusteredRedisPool { pool };
        SvixRedisPool::NonClustered(pool)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::cfg::CacheType;

    async fn get_pool(redis_dsn: &str, cache_type: &crate::cfg::CacheType) -> SvixRedisPool {
        match cache_type {
            CacheType::RedisCluster => {
                let mgr = crate::redis::create_redis_pool(redis_dsn, true).await;
                mgr
            }
            _ => {
                let mgr = crate::redis::create_redis_pool(redis_dsn, false).await;
                mgr
            }
        }
    }

    // Ensure basic set/get works -- should test sharding as well:
    #[tokio::test]
    async fn test_set_read_random_keys() {
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let pool = get_pool(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg.cache_type).await;
        let mut pool = pool.get().await.unwrap();

        for (val, key) in "abcdefghijklmnopqrstuvwxyz".chars().enumerate() {
            let key = key.to_string();
            pool.query_async::<()>(redis::Cmd::set::<String, usize>(key.clone(), val))
                .await
                .unwrap();
            assert_eq!(
                pool.query_async::<usize>(redis::Cmd::get(&key))
                    .await
                    .unwrap(),
                val
            );
        }

    }
}