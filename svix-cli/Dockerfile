# syntax=docker/dockerfile:1

# NOTE: this should be built from the root of `svix-webhooks`
# with `docker build -f svix-cli/Dockerfile .`# Base build

FROM docker.io/rust:1.97.1-alpine3.24 AS build
RUN cargo install --locked cargo-sbom@0.10.0

SHELL ["/bin/sh", "-eux", "-c"]
RUN --mount=target=/var/cache/apk,type=cache,sharing=locked --mount=target=/tmp,type=cache,sharing=locked <<EOF
    apk update
    apk add build-base~0.5 \
        curl~8 \
        openssl-dev~3.5 \
        zlib-dev~1.3 \
        cmake~4 \
        shadow~4 \
        git~2
EOF

RUN <<EOF
    mkdir -p /app
    useradd appuser
    chown -R appuser: /app
    mkdir -p /home/appuser
    chown -R appuser: /home/appuser
EOF

WORKDIR /app/svix-cli

COPY rust /app/rust
COPY svix-cli /app/svix-cli

RUN cargo build --release
RUN cargo sbom > /app/svix-cli.spdx
RUN strip target/release/svix

FROM alpine:3.24 AS checker
RUN mkdir /scripts
COPY bridge/scripts/check-deps.sh /scripts/check-deps.sh

# Production
FROM alpine:3.24 AS prod

SHELL ["/bin/sh", "-eux", "-c"]

RUN --mount=target=/var/cache/apk,type=cache,sharing=locked --mount=target=/tmp,type=cache,sharing=locked <<EOF
    apk add shadow~4 ca-certificates~20260611
    update-ca-certificates
EOF

RUN <<EOF
    mkdir -p /app
    useradd appuser
    chown -R appuser: /app
    mkdir -p /home/appuser
    chown -R appuser: /home/appuser
EOF

USER appuser

COPY --chown=root:root --chmod=755 --from=build /app/svix-cli/target/release/svix /usr/local/bin/svix
COPY --chown=root:root --chmod=444 --from=build /app/svix-cli.spdx /svix-cli.spdx

# Verify all the dynamic libs we depend on are present in the runtime stage
RUN --mount=type=bind,from=checker,source=/scripts,target=/scripts /scripts/check-deps.sh /usr/local/bin/svix

LABEL org.opencontainers.image.authors="support@svix.com" \
      org.opencontainers.image.url="https://www.svix.com" \
      org.opencontainers.image.documentation="https://docs.svix.com" \
      org.opencontainers.image.description="The Svix webhooks service, CLI component (OSS build)" \
      org.opencontainers.image.title="svix-webhooks, cli, oss" \
      org.opencontainers.image.vendor="Svix" \
      org.opencontainers.image.licenses="MIT" \
      org.opencontainers.image.base.name="docker.io/alpine:3.24"

CMD ["/usr/local/bin/svix"]
