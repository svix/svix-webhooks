# Base build
FROM rust:1.58-alpine3.15 AS build

RUN set -ex ; \
        mkdir -p /app ;\
        addgroup -S appgroup ;\
        adduser -S appuser -G appgroup ;\
        chown -R appuser: /app ;\
        mkdir -p /home/appuser ;\
        chown -R appuser: /home/appuser

RUN apk add --no-cache musl-dev

USER appuser

WORKDIR /app
COPY . .

RUN cd svix-server && cargo install --path .

# Production
FROM alpine:3.15 AS prod
# It can probably be scratch, just need to statically link musl

RUN set -ex ; \
        addgroup -S appgroup ;\
        adduser -S appuser -G appgroup ;\
        mkdir -p /home/appuser ;\
        chown -R appuser: /home/appuser

USER appuser
EXPOSE 8080

COPY --from=build /usr/local/cargo/bin/svix-server /usr/local/bin/svix-server

CMD "svix-server"
