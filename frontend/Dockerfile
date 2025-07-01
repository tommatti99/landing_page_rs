FROM rust:bullseye AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev curl

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown

RUN cargo install trunk

COPY . .

RUN trunk build --release

FROM nginx:latest

RUN rm -rf /usr/share/nginx/html/*

COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
