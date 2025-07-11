FROM rust:slim-bullseye AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    openssl \
    pkg-config \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    perl \
    make \
    gcc \
    g++ \
    && update-ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

COPY Cargo.toml Cargo.lock  ./

RUN mkdir -p src && echo "fn main() {}" > src/main.rs

RUN cargo fetch

RUN rm -rf src 

COPY ./src ./src

RUN cargo build --release

FROM debian:bullseye-slim AS prod-stage

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && update-ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

COPY --from=builder /app/target/release/backend .
COPY --from=builder /app/Cargo.toml /app/Cargo.lock ./

ENV RUST_BACKTRACE=full
EXPOSE 8080

RUN echo "Tamanhos dos arquivos:" && du -sh *

ENTRYPOINT ["./backend"]