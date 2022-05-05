# Base build
FROM rust:1.60-slim-bullseye AS build

RUN set -ex ; \
        mkdir -p /app ;\
        useradd appuser ;\
        chown -R appuser: /app ;\
        mkdir -p /home/appuser ;\
        chown -R appuser: /home/appuser

USER appuser

WORKDIR /app
COPY . .

WORKDIR /app/svix-server
RUN cargo install --path .

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

COPY ./wait-for /usr/local/bin/wait-for
COPY --from=build /usr/local/cargo/bin/svix-server /usr/local/bin/svix-server

# Ignoring this lint, because it's an embedded shell script
# hadolint ignore=DL3025
CMD \
    set -ex ; \
    if [ ! -z "$WAIT_FOR" ]; then \
        wait-for -t 15 pgbouncer:5432 -- echo "Postgres ready" ; \
        wait-for -t 15 redis:6379 -- echo "Redis ready" ; \
    fi ; \
    svix-server
