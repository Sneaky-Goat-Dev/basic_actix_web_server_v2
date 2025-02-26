# Stage 1: Build the application in release mode
FROM rust:1.84.1 as builder
WORKDIR /usr/src/basic_actix_web_server_v2
COPY . .
RUN cargo build --release

# Stage 2: Create a minimal image
FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/basic_actix_web_server/target/release/basic_actix_web_server_v2 /usr/local/bin/basic_actix_web_server_v2
CMD ["basic_actix_web_server"]