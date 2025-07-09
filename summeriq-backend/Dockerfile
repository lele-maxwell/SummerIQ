# ---- Build Stage ----
FROM rust:1.77 as builder
WORKDIR /app

# Install build dependencies for musl and OpenSSL
RUN apt-get update && apt-get install -y musl-tools pkg-config libssl-dev ca-certificates

# Set environment for musl
ENV OPENSSL_STATIC=1
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Copy source and manifest
COPY Cargo.toml Cargo.lock ./
COPY src ./src


# Build for musl (static binary)
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl || \
    cargo build --release

# ---- Runtime Stage ----
FROM debian:buster-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy built binary from builder (prefer static, fallback to dynamic)
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/summeriq-backend /app/summeriq-backend 2>/dev/null || \
    cp /app/target/release/summeriq-backend /app/summeriq-backend
COPY storage ./storage

EXPOSE 8080

CMD ["/app/summeriq-backend"] 