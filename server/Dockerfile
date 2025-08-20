# syntax=docker/dockerfile:1
#
# NOTE: this should be built from the server/ subdirectory of `svix-webhooks`
# with `docker build .`

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

SHELL ["/bin/bash", "-eux", "-o", "pipefail", "-c"]
RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked --mount=target=/var/cache/apt,type=cache,sharing=locked <<EOF
    export DEBIAN_FRONTEND=noninteractive
    apt-get update -q
    apt-get install -y \
        build-essential=12.* \
        checkinstall=1.* \
        zlib1g-dev=1:* \
        pkg-config=1.8.* \
        libssl-dev=* \
        --no-install-recommends
EOF

RUN <<EOF
    mkdir -p /app
    useradd appuser
    chown -R appuser: /app
    mkdir -p /home/appuser
    chown -R appuser: /home/appuser
EOF

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer
RUN cargo chef cook --release --package svix-server --bin svix-server --recipe-path recipe.json

# Build the server
COPY . .
RUN cargo build --release --package svix-server --bin svix-server --frozen

# Production
FROM docker.io/debian:trixie-slim AS prod

SHELL ["/bin/bash", "-eux", "-o", "pipefail", "-c"]
RUN <<EOF
    mkdir -p /app
    useradd appuser
    chown -R appuser: /app
    mkdir -p /home/appuser
    chown -R appuser: /home/appuser
EOF

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked --mount=target=/var/cache/apt,type=cache,sharing=locked <<EOF
    apt-get update -q
    apt-get install -y \
        ca-certificates=20250419 \
        libssl3t64=3.* \
        --no-install-recommends
    update-ca-certificates
EOF

USER appuser
EXPOSE 8071

COPY --chown=root:root --chmod=755 --from=build /app/target/release/svix-server /usr/local/bin/svix-server
COPY --chown=root:root --chmod=755 ./scripts/launch-svix-server /usr/local/bin/launch-svix-server

CMD ["/usr/local/bin/launch-svix-server"]
