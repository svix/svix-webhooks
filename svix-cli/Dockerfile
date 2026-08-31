# syntax=docker/dockerfile:1

# NOTE: this should be built from the root of `svix-webhooks`
# with `docker build -f svix-cli/Dockerfile .`# Base build

FROM cgr.dev/chainguard/wolfi-base@sha256:57108e597a8cf3bd376b810f1c3539c21942daefa242cb9dddaae30f8aac735d AS build
SHELL ["/bin/sh", "-eux", "-c"]
RUN --mount=target=/var/cache/apk,type=cache,sharing=locked --mount=target=/tmp,type=cache,sharing=locked <<EOF
    apk add rust~1.96.1 \
        build-base~1 \
        openssl-dev~3.6 \
        zlib-dev~1.3 \
        cmake~4 \
        shadow~4 \
        git~2
EOF

RUN cargo install --locked cargo-sbom@0.10.0

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

RUN <<EOF
    cargo build --release
    cargo sbom > /app/svix-cli.spdx
    strip target/release/svix
EOF

# ensure that the check-deps.sh script never ends up in the real image
FROM alpine:3.24 AS checker
RUN mkdir /scripts
COPY bridge/scripts/check-deps.sh /scripts/check-deps.sh

# Production
FROM cgr.dev/chainguard/wolfi-base@sha256:a31344ab2cb8618db84f535eec56f76f6178b142cb92cb2e48676cc2dcebea72 AS prod

SHELL ["/bin/sh", "-eux", "-c"]

RUN --mount=target=/var/cache/apk,type=cache,sharing=locked --mount=target=/tmp,type=cache,sharing=locked <<EOF
    apk add ca-certificates~20260413
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
      org.opencontainers.image.base.name="cgr.dev/chainguard/wolfi-base" \
      org.opencontainers.image.source=""

USER 1000:1000

CMD ["/usr/local/bin/svix"]
