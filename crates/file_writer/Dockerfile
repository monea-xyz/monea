FROM rust:1.67 AS builder

WORKDIR /usr/src/file_writer

COPY Cargo.toml ./

COPY src ./src

RUN cargo build --release

FROM debian:buster-slim

# Install OpenSSL - commonly required for Rust applications
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built executable from the builder stage
COPY --from=builder /usr/src/file_writer/target/release/file_writer /usr/local/bin/file_writer

# Set the startup command to run the binary
CMD ["artifact_writer"]