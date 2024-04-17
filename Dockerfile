FROM rust:latest as builder

WORKDIR /usr/src/decensha
COPY . .
RUN cargo build --release

FROM debian:bookworm
ENV DEBIAN_FRONTEND=noninteractive
COPY --from=builder /usr/src/decensha/target/release/decensha /usr/local/bin/decensha
RUN apt-get update && \
    apt-get install -y \
    build-essential \
    curl \
    postgresql \
    libpq-dev \
    pkg-config \
    openssl \
    libssl-dev

EXPOSE 7810
CMD ["decensha"]