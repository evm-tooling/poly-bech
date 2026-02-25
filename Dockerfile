# Poly-bench dev environment: Rust + Node.js for grammar, extension, and poly-bench build
FROM rust:1-bookworm

# Install Node.js 20 via NodeSource
RUN apt-get update && apt-get install -y curl ca-certificates \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Verify installations
RUN rustc --version && cargo --version && node --version && npm --version

WORKDIR /app
