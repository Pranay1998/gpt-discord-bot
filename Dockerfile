####################################################################################################
## Builder
####################################################################################################
FROM ubuntu:20.04 AS builder

ENV DEBIAN_FRONTEND noninteractive

WORKDIR /rust-app

COPY . .

RUN apt-get update && apt-get install -y \
    cmake \
    build-essential \
    curl \
    openssl \
    make \
    libssl-dev \
    pkg-config \
    yt-dlp \
    libopus-dev

# Get Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build --release

CMD ["./target/release/main"]
