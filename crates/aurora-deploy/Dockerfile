# =============================================================================
# AURORA NAV / GMIN — Production Dockerfile
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
RUN cargo build --release --bin aurora-nav \
    && strip target/release/aurora-nav

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
COPY --from=builder /build/target/release/aurora-nav /app/aurora-nav

# Copy default configuration (if present)
RUN cp -r /build/crates/aurora-config/defaults/ /app/config/ 2>/dev/null || true

# Set ownership
RUN chown -R aurora:aurora /app

USER aurora

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

ENTRYPOINT ["/app/aurora-nav"]
CMD ["--log-level", "info"]
