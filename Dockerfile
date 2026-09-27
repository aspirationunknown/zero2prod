# Using the latest stable release of Rust
FROM rust:1.98.0

# Switch to app working directory
WORKDIR /app

# Install required system dependencies
RUN apt update && apt install lld clang -y

# Copy files from working environment to Docker image
COPY . .

# Build binary in release
RUN cargo build --release

# When 'docker run' is executed, launch the application
ENTRYPOINT [ "./target/release/zero2prod" ]
