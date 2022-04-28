mod cluster;

use bb8::{Pool, RunError};
use bb8_redis::RedisConnectionManager;
pub use cluster::RedisClusterConnectionManager;

use axum::async_trait;
use redis::{FromRedisValue, RedisError, RedisResult};

#[derive(Clone, Debug)]
pub enum RedisPool {
    Clustered(ClusteredRedisPool),
    NonClustered(NonClusteredRedisPool),
}

#[derive(Clone, Debug)]
pub struct ClusteredRedisPool {
    pool: Pool<RedisClusterConnectionManager>,
}

#[derive(Clone, Debug)]
pub struct NonClusteredRedisPool {
    pool: Pool<RedisConnectionManager>,
}

pub enum PooledConnection<'a> {
    Clustered(ClusteredPooledConnection<'a>),
    NonClustered(NonClusteredPooledConnection<'a>),
}

impl<'a> PooledConnection<'a> {
    pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        match self {
            Self::Clustered(pooled_con) => pooled_con.query_async(cmd).await,
            Self::NonClustered(pooled_con) => pooled_con.query_async(cmd).await,
        }
    }
}

pub struct NonClusteredPooledConnection<'a> {
    con: bb8::PooledConnection<'a, RedisConnectionManager>,
}

impl<'a> NonClusteredPooledConnection<'a> {
    pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        cmd.query_async(&mut *self.con).await
    }
}

pub struct ClusteredPooledConnection<'a> {
    con: bb8::PooledConnection<'a, RedisClusterConnectionManager>,
}

impl<'a> ClusteredPooledConnection<'a> {
    pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        cmd.query_async(&mut *self.con).await
    }
}

#[async_trait]
pub trait PoolLike {
    async fn get(&self) -> Result<PooledConnection, RunError<RedisError>>;
}

#[async_trait]
impl PoolLike for RedisPool {
    async fn get(&self) -> Result<PooledConnection, RunError<RedisError>> {
        match self {
            Self::Clustered(pool) => pool.get().await,
            Self::NonClustered(pool) => pool.get().await,
        }
    }
}

#[async_trait]
impl PoolLike for NonClusteredRedisPool {
    async fn get(&self) -> Result<PooledConnection, RunError<RedisError>> {
        let con = self.pool.get().await?;
        let con = NonClusteredPooledConnection { con };
        Ok(PooledConnection::NonClustered(con))
    }
}

#[async_trait]
impl PoolLike for ClusteredRedisPool {
    async fn get(&self) -> Result<PooledConnection, RunError<RedisError>> {
        let con = ClusteredPooledConnection {
            con: self.pool.get().await?,
        };
        Ok(PooledConnection::Clustered(con))
    }
}

pub async fn create_redis_pool(redis_dsn: &str, clustered: bool) -> RedisPool {
    if clustered {
        let mgr = RedisClusterConnectionManager::new(redis_dsn)
            .expect("Error initializing redis cluster client");
        let pool = bb8::Pool::builder()
            .build(mgr)
            .await
            .expect("Error initializing redis cluster connection pool");
        let pool = ClusteredRedisPool { pool };
        RedisPool::Clustered(pool)
    } else {
        let mgr = RedisConnectionManager::new(redis_dsn).expect("Error intializing redis client");
        let pool = bb8::Pool::builder()
            .build(mgr)
            .await
            .expect("Error initializing redis connection pool");
        let pool = NonClusteredRedisPool { pool };
        RedisPool::NonClustered(pool)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::cfg::CacheType;

    async fn get_pool(redis_dsn: &str, cache_type: &crate::cfg::CacheType) -> RedisPool {
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