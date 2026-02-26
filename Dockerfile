# Build stage
FROM rust:1.85 AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*

# Copy manifests and build dependencies with dummy source
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && echo "" > src/lib.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source and rebuild
COPY src ./src
RUN touch src/main.rs src/lib.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/object_store_rust /usr/local/bin/object_store_rust

ENV STORE_PATH=/data/store
ENV DB_PATH=/data/db

VOLUME ["/data/store", "/data/db"]

EXPOSE 3000

CMD ["object_store_rust"]

# example run command:
# docuker run -p 3000:3000 -v /mnt/disk/store:/data/store -v /mnt/ssd/db:/data/db object-store
