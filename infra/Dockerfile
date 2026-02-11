# CI image watch_tower

FROM rust:1.93-slim-bookworm

LABEL org.opencontainers.image.source="https://github.com/cr0wSec/watch_tower"
LABEL org.opencontainers.image.description="CI image for watch_tower"

# disable interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# System dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev \
    pkg-config \
    libssl-dev \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Rust components
RUN rustup component add clippy rustfmt

# Install all the tools needed for CI
RUN cargo install diesel_cli --no-default-features --features postgres --locked \
    && cargo install cargo-tarpaulin --locked \
    && cargo install cargo-audit --locked \
    && cargo install cargo-deny --locked

# Cleanup cargo cache to reduce size
RUN rm -rf /usr/local/cargo/registry/cache \
    && rm -rf /usr/local/cargo/registry/src \
    && rm -rf /usr/local/cargo/git

# Check that everything is installed
RUN rustc --version \
    && cargo --version \
    && clippy-driver --version \
    && rustfmt --version \
    && diesel --version \
    && cargo-tarpaulin --version \
    && cargo-audit --version \
    && cargo-deny --version

WORKDIR /app