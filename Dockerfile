# GridTokenX Blockchain Node Docker Image
FROM rust:1.75-slim-bullseye as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libclang-dev \
    cmake \
    git \
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
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    curl \
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
EXPOSE 8080 8545 30303

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD gridtokenx-node status || exit 1

# Default command
CMD ["gridtokenx-node", "start"]
