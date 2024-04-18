FROM rust:latest as builder

WORKDIR /usr/src/decensha

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release

RUN rm -rf src
COPY src ./src
RUN cargo build --release

FROM debian:bookworm

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    curl \
    postgresql \
    libpq-dev \
    pkg-config \
    openssl \
    libssl-dev && \
    rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/decensha
COPY --from=builder /usr/src/decensha .

EXPOSE 7810
CMD ["cargo", "run", "--release"]