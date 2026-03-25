# ============================================================================
# AURORA NAV — Multi-stage Docker build
# ============================================================================
# Stage 1: Build the release binary
FROM rust:1.83-slim-bookworm AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/

RUN cargo build --release -p aurora-app && \
    strip target/release/aurora-nav

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash aurora

COPY --from=builder /build/target/release/aurora-nav /usr/local/bin/aurora-nav

USER aurora
WORKDIR /home/aurora

EXPOSE 3000

HEALTHCHECK --interval=15s --timeout=5s --start-period=10s --retries=3 \
    CMD ["/usr/local/bin/aurora-nav", "--status"]

ENTRYPOINT ["/usr/local/bin/aurora-nav"]
CMD ["--port", "3000"]
