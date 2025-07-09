# ---- Build Stage ----
FROM rust:1.77 as builder
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:buster-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy built binary from builder
COPY --from=builder /app/target/release/summeriq-backend /app/summeriq-backend

COPY migrations ./migrations
COPY storage ./storage

EXPOSE 8080

CMD ["/app/summeriq-backend"] 