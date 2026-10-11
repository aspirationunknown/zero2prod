# Build stage
# Using the latest stable release of Rust
FROM rust:1.98.0 AS builder

# Switch to app working directory
WORKDIR /app

# Install required system dependencies
RUN apt update && apt install lld clang -y

# Copy files from working environment to Docker image
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
ENTRYPOINT [ "./target/release/zero2prod" ]
