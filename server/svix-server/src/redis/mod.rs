mod cluster;

use bb8::{Pool, RunError};
use bb8_redis::RedisConnectionManager;
pub use cluster::RedisClusterConnectionManager;

use axum::async_trait;
use redis::{FromRedisValue, RedisError, RedisResult, ToRedisArgs};

use crate::cfg::Configuration;

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

#[async_trait]
pub trait PooledConnectionLike {
    async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T>;
    async fn query_async_pipeline<T: FromRedisValue>(
        &mut self,
        pipe: redis::Pipeline,
    ) -> RedisResult<T>;

    async fn del<K: ToRedisArgs + Send, T: FromRedisValue>(&mut self, key: K) -> RedisResult<T> {
        self.query_async(redis::Cmd::del(key)).await
    }

    async fn get<K: ToRedisArgs + Send, T: FromRedisValue>(&mut self, key: K) -> RedisResult<T> {
        let mut cmd = redis::cmd(if key.is_single_arg() { "GET" } else { "MGET" });
        cmd.arg(key);
        self.query_async(cmd).await
    }

    async fn lpop<K: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        count: Option<core::num::NonZeroUsize>,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::lpop(key, count)).await
    }

    async fn lrange<K: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        start: isize,
        stop: isize,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::lrange(key, start, stop)).await
    }

    async fn lrem<K: ToRedisArgs + Send, V: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        count: isize,
        value: V,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::lrem(key, count, value)).await
    }

    async fn pset_ex<K: ToRedisArgs + Send, V: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        value: V,
        milliseconds: usize,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::pset_ex(key, value, milliseconds))
            .await
    }

    async fn rpush<K: ToRedisArgs + Send, V: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        value: V,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::rpush(key, value)).await
    }

    async fn set<K: ToRedisArgs + Send, V: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        value: V,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::set(key, value)).await
    }

    async fn zadd<
        K: ToRedisArgs + Send,
        S: ToRedisArgs + Send,
        M: ToRedisArgs + Send,
        T: FromRedisValue,
    >(
        &mut self,
        key: K,
        member: M,
        score: S,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::zadd(key, member, score)).await
    }

    async fn zadd_multiple<
        K: ToRedisArgs + Send,
        S: ToRedisArgs + Send + Sync,
        M: ToRedisArgs + Send + Sync,
        T: FromRedisValue,
    >(
        &mut self,
        key: K,
        items: &'_ [(S, M)],
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::zadd_multiple(key, items))
            .await
    }

    async fn zpopmin<K: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        count: isize,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::zpopmin(key, count)).await
    }

    async fn zrange_withscores<K: ToRedisArgs + Send, T: FromRedisValue>(
        &mut self,
        key: K,
        start: isize,
        stop: isize,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::zrange_withscores(key, start, stop))
            .await
    }

    async fn zrangebyscore_limit<
        K: ToRedisArgs + Send,
        M: ToRedisArgs + Send,
        MM: ToRedisArgs + Send,
        T: FromRedisValue,
    >(
        &mut self,
        key: K,
        min: M,
        max: MM,
        offset: isize,
        count: isize,
    ) -> RedisResult<T> {
        self.query_async(redis::Cmd::zrangebyscore_limit(
            key, min, max, offset, count,
        ))
        .await
    }
}

#[async_trait]
impl<'a> PooledConnectionLike for PooledConnection<'a> {
    async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        match self {
            Self::Clustered(pooled_con) => pooled_con.query_async(cmd).await,
            Self::NonClustered(pooled_con) => pooled_con.query_async(cmd).await,
        }
    }

    async fn query_async_pipeline<T: FromRedisValue>(
        &mut self,
        pipe: redis::Pipeline,
    ) -> RedisResult<T> {
        match self {
            Self::Clustered(pooled_con) => pooled_con.query_async_pipeline(pipe).await,
            Self::NonClustered(pooled_con) => pooled_con.query_async_pipeline(pipe).await,
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

    pub async fn query_async_pipeline<T: FromRedisValue>(
        &mut self,
        pipe: redis::Pipeline,
    ) -> RedisResult<T> {
        pipe.query_async(&mut *self.con).await
    }
}

pub struct ClusteredPooledConnection<'a> {
    con: bb8::PooledConnection<'a, RedisClusterConnectionManager>,
}

impl<'a> ClusteredPooledConnection<'a> {
    pub async fn query_async<T: FromRedisValue>(&mut self, cmd: redis::Cmd) -> RedisResult<T> {
        cmd.query_async(&mut *self.con).await
    }

    pub async fn query_async_pipeline<T: FromRedisValue>(
        &mut self,
        pipe: redis::Pipeline,
    ) -> RedisResult<T> {
        pipe.query_async(&mut *self.con).await
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

async fn new_redis_pool_helper(
    redis_dsn: &str,
    clustered: bool,
    max_connections: u16,
) -> RedisPool {
    if clustered {
        let mgr = RedisClusterConnectionManager::new(redis_dsn)
            .expect("Error initializing redis cluster client");
        let pool = bb8::Pool::builder()
            .max_size(max_connections.into())
            .build(mgr)
            .await
            .expect("Error initializing redis cluster connection pool");
        let pool = ClusteredRedisPool { pool };
        RedisPool::Clustered(pool)
    } else {
        let mgr = RedisConnectionManager::new(redis_dsn).expect("Error intializing redis client");
        let pool = bb8::Pool::builder()
            .max_size(max_connections.into())
            .build(mgr)
            .await
            .expect("Error initializing redis connection pool");
        let pool = NonClusteredRedisPool { pool };
        RedisPool::NonClustered(pool)
    }
}

pub async fn new_redis_pool_clustered(redis_dsn: &str, cfg: &Configuration) -> RedisPool {
    new_redis_pool_helper(redis_dsn, true, cfg.redis_pool_max_size).await
}

pub async fn new_redis_pool(redis_dsn: &str, cfg: &Configuration) -> RedisPool {
    new_redis_pool_helper(redis_dsn, false, cfg.redis_pool_max_size).await
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::cfg::CacheType;

    async fn get_pool(redis_dsn: &str, cfg: &Configuration) -> RedisPool {
        match cfg.cache_type {
            CacheType::RedisCluster => crate::redis::new_redis_pool_clustered(redis_dsn, cfg).await,
            _ => crate::redis::new_redis_pool(redis_dsn, cfg).await,
        }
    }

    // Ensure basic set/get works -- should test sharding as well:
    #[tokio::test]
    async fn test_set_read_random_keys() {
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();

        let pool = get_pool(cfg.redis_dsn.as_ref().unwrap().as_str(), &cfg).await;
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
