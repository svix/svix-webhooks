# Base build
FROM rust:1.60-slim-bullseye AS build

RUN set -ex ; \
        mkdir -p /app ;\
        useradd appuser ;\
        chown -R appuser: /app ;\
        mkdir -p /home/appuser ;\
        chown -R appuser: /home/appuser

WORKDIR /app

# Hack to enable docker caching
COPY Cargo.toml .
COPY svix-server_derive svix-server_derive
COPY svix-server/Cargo.toml svix-server/
RUN set -ex ;\
        mkdir svix-server/src ;\
        echo 'fn main() { println!("Dummy!"); }' > svix-server/src/main.rs ;\
        cargo build --release ;\
        rm -rf svix-server/src

COPY . .
RUN cargo build --release --frozen

# Production
FROM debian:11.2-slim AS prod

RUN set -ex ; \
        mkdir -p /app ;\
        useradd appuser ;\
        chown -R appuser: /app ;\
        mkdir -p /home/appuser ;\
        chown -R appuser: /home/appuser

RUN apt-get update ;\
    apt-get install --no-install-recommends -y netcat=1.10-46 ;\
    apt-get clean ;\
    rm -rf /var/lib/apt/lists/*

USER appuser
EXPOSE 8080

COPY --from=build /app/target/release/svix-server /usr/local/bin/svix-server

# Ignoring this lint, because it's an embedded shell script
# hadolint ignore=DL3025
CMD \
    set -ex ; \
    if [ ! -z "$WAIT_FOR" ]; then \
		WAIT_FOR_ARG="--wait-for 15"; \
    fi ; \
    svix-server --run-migrations $WAIT_FOR_ARG
