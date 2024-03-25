# Base build
FROM rust:1.72-slim-bullseye AS build

RUN apt-get update && apt-get install -y \
    build-essential=12.* \
    checkinstall=1.* \
    curl=7.* \
    libssl-dev=* \
    pkg-config=0.29.* \
    zlib1g-dev=1:* \
    --no-install-recommends

RUN set -ex ; \
        mkdir -p /app ;\
        useradd appuser ;\
        chown -R appuser: /app ;\
        mkdir -p /home/appuser ;\
        chown -R appuser: /home/appuser

WORKDIR /app

# Hack to enable docker caching
COPY Cargo.toml .
COPY Cargo.lock .
COPY svix-bridge-types/Cargo.toml svix-bridge-types/
COPY svix-bridge-plugin-queue/Cargo.toml svix-bridge-plugin-queue/
COPY svix-bridge/Cargo.toml svix-bridge/
RUN set -ex ;\
        mkdir svix-bridge-plugin-queue/src ;\
        mkdir svix-bridge-types/src ;\
        mkdir svix-bridge/src ;\
        echo '' > svix-bridge-plugin-queue/src/lib.rs ;\
        echo '' > svix-bridge-types/src/lib.rs ;\
        echo 'fn main() { println!("Dummy!"); }' > svix-bridge/src/main.rs ;\
        cargo build --release ;\
        rm -rf \
          svix-bridge-plugin-queue/src \
          svix-bridge-types/src \
          svix-bridge/src

COPY . .
# touching the lib.rs/main.rs ensures cargo rebuilds them instead of considering them already built.
RUN touch */src/lib.rs && touch */src/main.rs
RUN cargo build --release --frozen

# Production
FROM debian:bullseye-slim AS prod

RUN set -ex ; \
        mkdir -p /app ;\
        useradd appuser ;\
        chown -R appuser: /app ;\
        mkdir -p /home/appuser ;\
        chown -R appuser: /home/appuser

RUN apt-get update ;\
    apt-get install --no-install-recommends -y ca-certificates=20210119; \
    update-ca-certificates; \
    rm -rf /var/lib/apt/lists/*

USER appuser

COPY --from=build /app/target/release/svix-bridge /usr/local/bin/svix-bridge
EXPOSE 5000

# Will fail if there's no `svix-bridge.yaml` in the CWD or `SVIX_BRIDGE_CFG` is not set to a valid
# path to a config.
CMD ["svix-bridge"]
