# syntax=docker/dockerfile:1

# NOTE: this should be built from the root of `svix-webhooks`
# with `docker build -f bridge/Dockerfile .`

# Base build
FROM docker.io/rust:1.89-slim-trixie AS build

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

WORKDIR /app/bridge

# Hack to enable docker caching
COPY rust /app/rust
COPY bridge/Cargo.toml .
COPY bridge/Cargo.lock .
COPY bridge/svix-bridge-types/Cargo.toml svix-bridge-types/
COPY bridge/svix-bridge-plugin-kafka/Cargo.toml svix-bridge-plugin-kafka/
COPY bridge/svix-bridge-plugin-queue/Cargo.toml svix-bridge-plugin-queue/
COPY bridge/svix-bridge/Cargo.toml svix-bridge/

RUN <<EOF
    mkdir svix-bridge-plugin-kafka/src
    mkdir svix-bridge-plugin-queue/src
    mkdir svix-bridge-types/src
    mkdir svix-bridge/src
    echo '' > svix-bridge-plugin-kafka/src/lib.rs
    echo '' > svix-bridge-plugin-queue/src/lib.rs
    echo '' > svix-bridge-types/src/lib.rs
    echo 'fn main() { println!("Dummy!"); }' > svix-bridge/src/main.rs
    cargo build --release
    rm -rf \
        svix-bridge-plugin-queue/src \
        svix-bridge-types/src \
        svix-bridge/src
EOF

COPY bridge /app/bridge
# touching the lib.rs/main.rs ensures cargo rebuilds them instead of considering them already built.
RUN touch */src/lib.rs && touch */src/main.rs
RUN cargo build --release --frozen

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
    export DEBIAN_FRONTEND=noninteractive
    apt-get update -q
    apt-get install --no-install-recommends -y ca-certificates=20250419
    update-ca-certificates
EOF

USER appuser

COPY --from=build /app/bridge/target/release/svix-bridge /usr/local/bin/svix-bridge
COPY bridge/scripts/check-deps.sh /usr/local/bin/check-deps.sh

# Verify all the dynamic libs we depend on are present in the runtime stage
RUN /usr/local/bin/check-deps.sh /usr/local/bin/svix-bridge

EXPOSE 5000

# Will fail if there's no `svix-bridge.yaml` in the CWD or `SVIX_BRIDGE_CFG` is not set to a valid
# path to a config.
CMD ["svix-bridge"]
