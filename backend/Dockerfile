# ===============================================
#                COMPILAR PROJETO
# ===============================================
# versao minima do debian 12 com rust 
FROM rust:1.80-slim-bookworm AS builder

# diretorio inicial do container
WORKDIR /app

# copiar arquivos necessarios
COPY ..

# compilar o projeto
RUN cargo build --locked --release
# ====================================
 


# ===============================================
#                MONTAR CONTAINER
# ===============================================
# versao minima do debian 12 pro bin
FROM debian:bookworm-slim

# diretorio inicial do container
WORKDIR /app

# copia arquivos compilados
COPY --from=builder /app/target/release/backend

# executa binario
CMD ["./backend"]