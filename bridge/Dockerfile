# syntax=docker/dockerfile:1

# NOTE: this should be built from the root of `svix-webhooks`
# with `docker build -f bridge/Dockerfile .`

# Base build
FROM cgr.dev/chainguard/wolfi-base@sha256:0feab531471d2edfcca55f18883441cb7df598e724fa2dedf9985273f532eed7 AS build
SHELL ["/bin/sh", "-eux", "-c"]
RUN --mount=target=/var/cache/apk,type=cache,sharing=locked --mount=target=/tmp,type=cache,sharing=locked <<EOF
    apk add rust~1.98.1 \
        build-base~1 \
        openssl-dev~4 \
        zlib-dev~1.3 \
        cmake~4 \
        shadow~4 \
        cmake~3 \
        git~2 \
        curl~8
EOF

RUN cargo install --locked cargo-sbom@0.10.0

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

ARG WRITE_EXECUTABLE_METADATA="1"

# touching the lib.rs/main.rs ensures cargo rebuilds them instead of considering them already built.
RUN <<EOF
    touch ./*/src/lib.rs
    touch ./*/src/main.rs
    cargo build --release --frozen
    cargo sbom > /app/svix-bridge.spdx
EOF

# ensure that the check-deps.sh script never ends up in the real image
FROM alpine:3.24 AS checker
RUN mkdir /scripts
COPY bridge/scripts/check-deps.sh /scripts/check-deps.sh

# Production
FROM cgr.dev/chainguard/wolfi-base@sha256:0feab531471d2edfcca55f18883441cb7df598e724fa2dedf9985273f532eed7 AS base
SHELL ["/bin/sh", "-eux", "-c"]

RUN --mount=target=/var/cache/apk,type=cache,sharing=locked --mount=target=/tmp,type=cache,sharing=locked <<EOF
    apk update
    apk add \
        ca-certificates~20260909-r3 \
        openssl~4 \
        tini~0.19 \
        bash~5
    update-ca-certificates
EOF


RUN <<EOF
    mkdir -p /app
    addgroup -S -g 1000 appgroup
    adduser -G appgroup -S -u 1000 appuser
    chown -R appuser:appgroup /app
    mkdir -p /home/appuser
    chown -R appuser:appgroup /home/appuser
EOF

USER 1001

COPY --from=build /app/bridge/target/release/svix-bridge /usr/local/bin/svix-bridge
COPY --chown=root:root --chmod=644 --from=build /app/svix-bridge.spdx /svix-bridge.spdx

# Verify all the dynamic libs we depend on are present in the runtime stage
RUN --mount=type=bind,from=checker,source=/scripts,target=/scripts /scripts/check-deps.sh /usr/local/bin/svix-bridge

LABEL org.opencontainers.image.authors="support@svix.com" \
      org.opencontainers.image.url="https://www.svix.com" \
      org.opencontainers.image.documentation="https://docs.svix.com" \
      org.opencontainers.image.description="The Svix webhooks service, Bridge component (OSS build)" \
      org.opencontainers.image.title="svix-webhooks, bridge, oss" \
      org.opencontainers.image.vendor="Svix" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.base.name="cgr.dev/chainguard/wolfi-base"

EXPOSE 5000

ENTRYPOINT ["/usr/bin/tini", "--"]

# Will fail if there's no `svix-bridge.yaml` in the CWD or `SVIX_BRIDGE_CFG` is not set to a valid
# path to a config.
CMD ["svix-bridge"]
