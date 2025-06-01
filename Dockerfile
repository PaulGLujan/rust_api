# --- Builder Stage ---
FROM rust:1.87-slim-bullseye AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    musl-tools \
    && rm -rf /var/lib/apt/lists/* # Clean up apt cache to keep image small

COPY Cargo.toml Cargo.lock ./

RUN rustup target add x86_64-unknown-linux-musl

COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

# --- Runtime Stage ---
FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust_api .

EXPOSE 3000

CMD ["./rust_api"]