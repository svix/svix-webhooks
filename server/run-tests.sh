#!/bin/sh -e

# Run tests with various configurations:

# Common variables:
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/postgres"
export SVIX_JWT_SECRET="test value"
export SVIX_LOG_LEVEL="info"
export SVIX_WHITELIST_SUBNETS="[127.0.0.1/32]"
export SVIX_DB_POOL_MAX_SIZE="500"
export SVIX_REDIS_POOL_MAX_SIZE="10000"

echo "*********** RUN 1 ***********"
(
    export SVIX_QUEUE_TYPE="redis"
    export SVIX_CACHE_TYPE="redis"
    export SVIX_REDIS_DSN="redis://localhost:6379"
    cargo test
    cargo test -- --ignored redis
)

echo "*********** RUN 2 ***********"
(
    export SVIX_QUEUE_TYPE="redis"
    export SVIX_CACHE_TYPE="memory"
    export SVIX_REDIS_DSN="redis://localhost:6379"
    cargo test
)

echo "*********** RUN 3 ***********"
(
    export SVIX_QUEUE_TYPE="redis"
    export SVIX_CACHE_TYPE="none"
    export SVIX_REDIS_DSN="redis://localhost:6379"
    cargo test
)

echo "*********** RUN 4 ***********"
(
    export SVIX_QUEUE_TYPE="rediscluster"
    export SVIX_CACHE_TYPE="rediscluster"
    export SVIX_REDIS_DSN="redis://localhost:6380"
    cargo test
    cargo test -- --ignored redis
)

echo "*********** RUN 5 ***********"
(
    export SVIX_QUEUE_TYPE="memory"
    export SVIX_CACHE_TYPE="none"
    cargo test
)

echo "*********** RUN 6 ***********"
(
    export SVIX_QUEUE_TYPE="rabbitmq"
    export SVIX_CACHE_TYPE="redis"
    export SVIX_REDIS_DSN="redis://localhost:6379"
    export SVIX_RABBIT_DSN="amqp://xivs:xivs@localhost:5672/%2f"
    cargo test
    cargo test -- --ignored rabbitmq
)
