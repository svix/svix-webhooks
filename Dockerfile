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

WORKDIR /app

COPY ./server/svix-server .

RUN cargo check

# Build the rust app
RUN cd server/svix-server && cargo build

##################################################################
FROM scratch

WORKDIR /app
# Import the user and group files from the builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy our binary
COPY --from=builder /app/server/svix-server/target/release/svix-server /app/svix-server

USER appuser:appuser

# Run the binary
ENTRYPOINT ["/app/svix-server"]
