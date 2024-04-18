FROM rust:latest as builder

RUN apt-get update && \
    apt-get install -y \
    build-essential \
    curl \
    postgresql \
    libpq-dev \
    pkg-config \
    openssl \
    libssl-dev

WORKDIR /usr/src/decensha

COPY src ./src
COPY Cargo.toml Cargo.lock ./
COPY .env ./.env
COPY .env.local ./.env.local
RUN cargo build --release

EXPOSE 7810
CMD ["./target/release/decensha"]