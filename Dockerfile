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

FROM rust:1.98.0-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/zero2prod zero2prod

COPY configuration configuration

# Set production for configuration
ENV APP_ENVIRONMENT="production"

# When 'docker run' is executed, launch the application
ENTRYPOINT [ "./target/release/zero2prod" ]
