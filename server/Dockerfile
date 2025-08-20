# Using https://github.com/LukeMathWalker/cargo-chef for better layer caching

# Base image for planner and build - keep in sync with .github/workflows/server-ci.yml
FROM docker.io/rust:1.89-slim-trixie AS chef
RUN cargo install cargo-chef
WORKDIR /app

# Build plan environment
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build environment
FROM chef AS build

RUN apt-get update && apt-get install -y \
        build-essential=12.* \
        checkinstall=1.* \
        zlib1g-dev=1:* \
        pkg-config=1.8.* \
        libssl-dev=* \
        --no-install-recommends

RUN set -ex && \
        mkdir -p /app && \
        useradd appuser && \
        chown -R appuser: /app && \
        mkdir -p /home/appuser && \
        chown -R appuser: /home/appuser

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer
RUN cargo chef cook --release --package svix-server --bin svix-server --recipe-path recipe.json

# Build the server
COPY . .
RUN cargo build --release --package svix-server --bin svix-server --frozen

# Production
FROM docker.io/debian:trixie-slim AS prod

RUN set -ex && \
        mkdir -p /app && \
        useradd appuser && \
        chown -R appuser: /app && \
        mkdir -p /home/appuser && \
        chown -R appuser: /home/appuser

RUN apt-get update && apt-get install -y \
        ca-certificates=20250419 \
        libssl3t64=3.* \
        --no-install-recommends && \
        update-ca-certificates && \
        rm -rf /var/lib/apt/lists/*

USER appuser
EXPOSE 8071

COPY --from=build /app/target/release/svix-server /usr/local/bin/svix-server

# Ignoring this lint, because it's an embedded shell script
# hadolint ignore=DL3025
CMD \
    set -ex ; \
    if [ ! -z "$WAIT_FOR" ]; then \
        WAIT_FOR_ARG="--wait-for 15"; \
    fi ; \
    exec svix-server --run-migrations $WAIT_FOR_ARG
