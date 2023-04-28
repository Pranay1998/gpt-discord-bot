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
    pkg-config \
    libssl-dev

# Get Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build --release


####################################################################################################
## Final Image 
####################################################################################################

FROM ubuntu:20.04

ENV DEBIAN_FRONTEND noninteractive

WORKDIR /rust-app

RUN apt-get update && apt-get install -y software-properties-common
RUN add-apt-repository ppa:tomtomtom/yt-dlp

RUN apt-get update && apt-get install -y \
    yt-dlp \
    ffmpeg \
    libopus-dev

COPY --from=builder /rust-app/target/release/main ./

CMD ["/rust-app/main"]
