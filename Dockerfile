# Build stage
# Using cargo-chef and the latest stable release of Rust
FROM lukemathwalker/cargo-chef:latest-rust-1.98.0 AS chef

# Switch to app working directory
WORKDIR /app

# Install required system dependencies
RUN apt update && apt install lld clang -y

FROM chef AS planner

# Copy files from working environment to Docker image
COPY . .

# Create a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

# Build project dependencies, not the application itself
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

# Set the SQLX_OFFLINE environment variable for offline compilation
ENV SQLX_OFFLINE=true

# Build binary in release
RUN cargo build --release

# Runtime stage
# Use a slimmer image as the base image for the runtime
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install OpenSSL and ca-certificates required for establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero2prod zero2prod

COPY configuration configuration

# Set production for configuration
ENV APP_ENVIRONMENT="production"

# When 'docker run' is executed, launch the application
ENTRYPOINT [ "/app/zero2prod" ]
