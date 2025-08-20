# Base build
FROM docker.io/rust:1.89-slim-trixie AS build

RUN apt-get update && apt-get install -y \
    build-essential=12.* \
    checkinstall=1.* \
    curl=8.* \
    libssl-dev=* \
    pkg-config=1.8.* \
    zlib1g-dev=1:* \
    cmake=3.* \
    --no-install-recommends

RUN set -ex && \
        mkdir -p /app && \
        useradd appuser && \
        chown -R appuser: /app && \
        mkdir -p /home/appuser && \
        chown -R appuser: /home/appuser

WORKDIR /app/svix-cli

# Hack to enable docker caching
COPY rust /app/rust
COPY svix-cli/Cargo.toml .
COPY svix-cli/Cargo.lock .

RUN set -ex && \
        mkdir src && \
        echo 'fn main() { println!("Dummy!"); }' > src/main.rs && \
        cargo build --release && \
        rm -rf \
          src

COPY . /app
RUN touch src/main.rs
RUN cargo build --release --frozen

# Production
FROM docker.io/debian:trixie-slim AS prod

RUN set -ex && \
        mkdir -p /app && \
        useradd appuser && \
        chown -R appuser: /app && \
        mkdir -p /home/appuser && \
        chown -R appuser: /home/appuser

RUN apt-get update && \
    apt-get install --no-install-recommends -y ca-certificates=20250419 && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

USER appuser

COPY --from=build /app/svix-cli/target/release/svix /usr/local/bin/svix
COPY bridge/scripts/check-deps.sh /usr/local/bin/check-deps.sh

# Verify all the dynamic libs we depend on are present in the runtime stage
RUN /usr/local/bin/check-deps.sh /usr/local/bin/svix

CMD ["svix"]
