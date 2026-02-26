# Build stage
FROM rust:1.85 AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

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
