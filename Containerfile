# Build frontend
FROM oven/bun:1 AS frontend
WORKDIR /app/client
# Copy package files first to leverage Docker caching
COPY ./client/package.json ./client/bun.lock ./
# Fix for ARM64
RUN apt-get update && apt-get install -y python3 make g++
RUN bun install --frozen-lockfile
# Now copy the rest of the files
COPY ./client/ ./
RUN bun run build --out-dir /app/dist

# Build rust binary
FROM rust:1.85-alpine3.20 as builder
# see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
# Install OpenSSL and other required dependencies
RUN apk add --no-cache musl-dev openssl-dev pkgconfig
WORKDIR /app
COPY ./ /app
RUN cargo build --release
RUN strip target/release/health

# Prod stage

# alpine version must match build stage
FROM alpine:3.20
RUN apk add --no-cache libgcc openssl
WORKDIR /app
COPY --from=builder /app/target/release/health ./
COPY --from=frontend /app/dist ./dist
ENTRYPOINT ["./health"]
