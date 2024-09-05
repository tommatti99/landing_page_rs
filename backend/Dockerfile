# ===============================================
#                COMPILAR PROJETO
# ===============================================
# Versão mínima do Debian 12 com Rust
FROM rust:bookworm as builder

# Diretório inicial do container
WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copiar todos os arquivos do diretório atual para o diretório de trabalho do container
COPY . .

# Compilar o projeto
RUN cargo build --locked --release
# ====================================
 

# ===============================================
#                MONTAR CONTAINER
# ===============================================
# Versão mínima do Debian 12 para o binário
FROM debian:bookworm-slim

# Diretório inicial do container
WORKDIR /app

# Copia o binário compilado
COPY --from=builder /app/target/release/backend /app/backend

# Executa o binário
CMD ["./backend"]
