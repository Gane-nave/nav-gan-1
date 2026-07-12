# =============================================================================
# G.A.N.E NAV — Production Dockerfile
# Multi-stage build for minimal runtime image
# =============================================================================

# Stage 1: Build
FROM rust:1.83-slim-bookworm AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests first for layer caching
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/

# Build release binary
RUN cargo build --release --bin gane-nav \
    && strip target/release/gane-nav

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 1001 aurora \
    && useradd --uid 1001 --gid aurora --shell /bin/false aurora

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /build/target/release/gane-nav /app/gane-nav

# Defaults are generated programmatically by gane-config; no config files needed.

# Set ownership
RUN chown -R aurora:aurora /app

USER gane

# Expose API port
EXPOSE 3000

# Health check using the /health endpoint
HEALTHCHECK --interval=15s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -sf http://localhost:3000/health || exit 1

# Environment variables
ENV AURORA_LOG_LEVEL=info \
    AURORA_LOG_FORMAT=json \
    AURORA_API_HOST=0.0.0.0 \
    AURORA_API_PORT=3000 \
    RUST_BACKTRACE=1

ENTRYPOINT ["/app/gane-nav"]
CMD ["--log-level", "info"]
