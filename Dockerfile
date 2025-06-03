# Dockerfile

# --- Stage 1: cargo-chef for dependency caching ---
# We use a specific Rust version (e.g., 1.87.0) to ensure consistency.
# Check your local `rustc --version` and use a matching `cargo-chef` image.
FROM rust:1.87.0-slim-bullseye AS chef

# Install cargo-chef
RUN cargo install cargo-chef --locked

WORKDIR /app

# Copy only Cargo.toml and Cargo.lock to leverage Docker's cache for dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Prepare the dependency recipe. This generates `recipe.json`
# If Cargo.toml/Cargo.lock haven't changed, this step (and subsequent `cook` step) will be cached.
RUN cargo chef prepare --recipe-path recipe.json

# --- Stage 2: Builder for compiling the application ---
# Use the same base image as chef for consistency
FROM rust:1.87.0-slim-bullseye AS builder

RUN cargo install cargo-chef --locked

# Install sqlx-cli in the builder stage for potential future use (e.g., if you run migrations inside Docker)
# Though for compile-time checks, copying .sqlx is preferred.
RUN cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /app

# Copy the generated recipe.json from the chef stage
COPY --from=chef /app/recipe.json ./recipe.json

COPY Cargo.toml Cargo.lock ./

# Use cargo-chef to "cook" (compile) the dependencies based on the recipe
# This step is heavily cached by Docker if recipe.json hasn't changed.
RUN cargo chef cook --recipe-path recipe.json

# Copy your source code
COPY src ./src

# Copy the pre-generated SQLx query cache. This avoids needing a database connection during build.
COPY .sqlx ./.sqlx

# Build the final release binary
# --locked ensures reproducible builds based on Cargo.lock
# --bin rust_api specifies the binary name if your project has multiple binaries
# (default is your package name, often 'rust_api' in this case)
RUN cargo build --release --bin rust_api

# --- Stage 3: Final lean production image ---
# Use a minimal Debian base image for the smallest possible final image
FROM debian:bullseye-slim

# Install ca-certificates for HTTPS communication (e.g., if connecting to cloud databases or external APIs)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
# Replace 'rust_api' with the actual name of your binary (usually your crate name)
COPY --from=builder /app/target/release/rust_api .

# Expose the port your Axum application listens on (default is 3000)
EXPOSE 3000

# Set the entrypoint for the application
# Environment variables (DATABASE_URL, JWT_SECRET) will be passed at runtime (docker run -e)
CMD ["./rust_api"]