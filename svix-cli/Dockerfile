# syntax=docker/dockerfile:1

# NOTE: this should be built from the root of `svix-webhooks`
# with `docker build -f svix-cli/Dockerfile .`# Base build

FROM docker.io/rust:1.96-slim-trixie AS build

SHELL ["/bin/bash", "-eux", "-o", "pipefail", "-c"]
RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked --mount=target=/var/cache/apt,type=cache,sharing=locked <<EOF
    export DEBIAN_FRONTEND=noninteractive
    apt-get update -q
    apt-get install -y \
        build-essential=12.* \
        checkinstall=1.* \
        curl=8.* \
        libssl-dev=* \
        pkg-config=1.8.* \
        zlib1g-dev=1:* \
        cmake=3.* \
        --no-install-recommends
EOF

RUN <<EOF
    mkdir -p /app
    useradd appuser
    chown -R appuser: /app
    mkdir -p /home/appuser
    chown -R appuser: /home/appuser
EOF

WORKDIR /app/svix-cli

# Hack to enable docker caching
COPY rust /app/rust
COPY svix-cli/Cargo.toml .
COPY svix-cli/Cargo.lock .

RUN <<EOF
    mkdir src
    echo 'fn main() { println!("Dummy!"); }' > src/main.rs
    cargo build --release
    rm -rf src
EOF

COPY . /app
RUN touch src/main.rs
RUN cargo build --release --frozen

# Production
FROM docker.io/debian:trixie-20260713-slim AS prod

SHELL ["/bin/bash", "-eux", "-o", "pipefail", "-c"]
RUN <<EOF
    mkdir -p /app
    useradd appuser
    chown -R appuser: /app
    mkdir -p /home/appuser
    chown -R appuser: /home/appuser
EOF

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked --mount=target=/var/cache/apt,type=cache,sharing=locked <<EOF
    export DEBIAN_FRONTEND=noninteractive
    apt-get update -q
    apt-get install -y --no-install-recommends ca-certificates=20250419
    update-ca-certificates
EOF

USER appuser

COPY --chown=root:root --chmod=755 --from=build /app/svix-cli/target/release/svix /usr/local/bin/svix
COPY bridge/scripts/check-deps.sh /usr/local/bin/check-deps.sh

# Verify all the dynamic libs we depend on are present in the runtime stage
RUN /usr/local/bin/check-deps.sh /usr/local/bin/svix

LABEL org.opencontainers.image.authors="support@svix.com" \
      org.opencontainers.image.url="https://www.svix.com" \
      org.opencontainers.image.documentation="https://docs.svix.com" \
      org.opencontainers.image.description="The Svix webhooks service, CLI component (OSS build)" \
      org.opencontainers.image.title="svix-webhooks, cli, oss" \
      org.opencontainers.image.vendor="Svix" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.base.name="docker.io/debian:trixie"

CMD ["/usr/local/bin/svix"]
