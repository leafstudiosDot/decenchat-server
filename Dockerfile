FROM ubuntu:22.04

WORKDIR /app
COPY . /app

RUN apt-get update

RUN apt-get install -y \
    build-essential \
    curl

RUN OPENSSL_DIR=/usr/bin/openssl DEBIAN_FRONTEND=noninteractive apt-get install -y openssl libssl-dev pkg-config libssl-dev

RUN apt-get update
RUN apt-get clean

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup update stable
RUN cargo build