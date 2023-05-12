#!/bin/sh -e

# Run tests with various configurations:

TEST_COMMAND="cargo test --all --all-features --all-targets $*"

# Common variables:
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/postgres"
export SVIX_JWT_SECRET="test value"
export SVIX_LOG_LEVEL="info"
export SVIX_WHITELIST_SUBNETS="[127.0.0.1/32]"

echo "*********** RUN 1 ***********"
SVIX_QUEUE_TYPE="redis" \
SVIX_CACHE_TYPE="redis" \
SVIX_REDIS_DSN="redis://localhost:6379" \
${TEST_COMMAND}

echo "*********** RUN 2 ***********"
SVIX_QUEUE_TYPE="redis" \
SVIX_CACHE_TYPE="memory" \
SVIX_REDIS_DSN="redis://localhost:6379" \
${TEST_COMMAND}

echo "*********** RUN 3 ***********"
SVIX_QUEUE_TYPE="redis" \
SVIX_CACHE_TYPE="none" \
SVIX_REDIS_DSN="redis://localhost:6379" \
${TEST_COMMAND}

echo "*********** RUN 4 ***********"
SVIX_QUEUE_TYPE="rediscluster" \
SVIX_CACHE_TYPE="rediscluster"  \
SVIX_REDIS_DSN="redis://localhost:6380" \
${TEST_COMMAND}

echo "*********** RUN 5 ***********"
SVIX_QUEUE_TYPE="memory" \
SVIX_CACHE_TYPE="none"  \
SVIX_REDIS_DSN="redis://localhost:6379" \
${TEST_COMMAND}

echo "*********** RUN 6 ***********"
SVIX_QUEUE_TYPE="rabbitmq" \
SVIX_CACHE_TYPE="redis" \
SVIX_REDIS_DSN="redis://localhost:6379" \
SVIX_RABBIT_DSN="amqp://xivs:xivs@localhost:5672/%2f" \
${TEST_COMMAND}
