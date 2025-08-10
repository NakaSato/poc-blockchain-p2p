# GridTokenX Blockchain Node Docker Image
# Version: 0.1.1 - Updated August 2025
# Production-ready containerized deployment for GridTokenX P2P energy trading platform
FROM rust:1.80-slim-bookworm AS builder

# Install system dependencies for Rust compilation and libp2p
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libclang-dev \
    llvm-dev \
    cmake \
    git \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy dependency files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src/ ./src/

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    netcat-traditional \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 gridtoken && \
    mkdir -p /app/data && \
    chown -R gridtoken:gridtoken /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/gridtokenx-node /usr/local/bin/gridtokenx-node

# Copy configuration
COPY config.toml /app/config.toml

# Switch to non-root user
USER gridtoken

# Set working directory
WORKDIR /app

# Create data directory
VOLUME ["/app/data"]

# Expose ports
# 8080: REST API server
# 9000: P2P networking (updated from 30303)
# 9090: Metrics/Prometheus
EXPOSE 8080 9000 9090

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/api/v1/health || exit 1

# Default command
CMD ["gridtokenx-node", "start", "--config", "/app/config.toml"]
