FROM rust:1.67 AS builder

# Create appuser.
ENV USER=appuser
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

RUN apk add -U --no-cache ca-certificates git

# RUN GOCACHE=OFF

# RUN go env -w GOPRIVATE=github.com/Bureau-Inc/*

# ARG githubAccessToken

# RUN git config --global url."https://golang:$githubAccessToken@github.com".insteadOf "https://github.com"

WORKDIR /app

COPY ./server/svix-server .

RUN cargo check

# Build the rust app
RUN cargo build svix_webhook_server ./cmd/

##################################################################
FROM scratch

WORKDIR /app
# Import the user and group files from the builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy our binary
COPY --from=builder /app/svix_webhook_server /app/svix_webhook_server
COPY --from=builder /app/.env /app/.env

USER appuser:appuser

# Run the binary
ENTRYPOINT ["/app/svix_webhook_server"]
