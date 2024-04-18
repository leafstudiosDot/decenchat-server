FROM rust:latest as chef
RUN cargo install cargo-chef --version 0.1.66 --root /usr/local/cargo

FROM rust:latest as planner
WORKDIR /usr/src/decensha
COPY --from=chef /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:latest as cacher
WORKDIR /usr/src/decensha
COPY --from=chef /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY --from=planner /usr/src/decensha/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:latest as builder
WORKDIR /usr/src/decensha
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY src ./src
COPY Cargo.toml Cargo.lock ./
COPY .env ./.env
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /usr/src/decensha
COPY --from=builder /usr/src/decensha/target/release/decensha /usr/src/decensha/target/release/decensha
RUN apt-get update && \
    apt-get install -y openssl libssl-dev && \
    rm -rf /var/lib/apt/lists/*

EXPOSE 7810
CMD ["./target/release/decensha"]