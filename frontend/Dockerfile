# ===============================================
#                COMPILAR PROJETO
# ===============================================
# versao minima do debian com rust
FROM rust:1.80.1-slim-bookworm AS builder

# diretorio inicial do container
WORKDIR /app

# copiar arquivos necessarios
COPY ..

# instala dependencias cargo
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown

# compilar o projeto
RUN trunk build --release --public-url landing_page/
# ===============================================
#                MONTAR CONTAINER
# ===============================================
# versao minima do debian pro container
FROM debian:bookworm-slim

# define diretorio de trabalho do container
WORKDIR /app

# copia arquivos da compilacao
COPY --from=builder /app/dist

# expor porta 80
EXPOSE 80