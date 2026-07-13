# ============================================================================
# G.A.N.E NAV — Multi-stage Docker build
# ============================================================================
# Stage 1: Build the release binary
FROM rust:1.83-slim-bookworm AS builder

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/

RUN cargo build --release -p gane-app && \
    strip target/release/gane-nav

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash gane

COPY --from=builder /build/target/release/gane-nav /usr/local/bin/gane-nav

USER gane
WORKDIR /home/aurora

EXPOSE 3000

HEALTHCHECK --interval=15s --timeout=5s --start-period=10s --retries=3 \
    CMD ["/usr/local/bin/gane-nav", "--status"]

ENTRYPOINT ["/usr/local/bin/gane-nav"]
CMD ["--port", "3000"]
