# Etapa de construção
FROM rust:1.80 as builder

# Definir o diretório de trabalho
WORKDIR /historic_sys

# Copiar o arquivo Cargo.toml e resolver dependências
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release && rm -rf src

# Copiar o código-fonte do projeto
COPY ./src ./src

# Construir o binário final em modo release
RUN cargo build --release

# Etapa de runtime
FROM debian:bookworm-slim

# Instalar as bibliotecas necessárias (libssl-dev, ca-certificates)
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Copiar o binário construído da etapa anterior
COPY --from=builder /historic_sys/target/release/historic_sys /usr/local/bin/historic_sys

# Definir o comando padrão
CMD ["/usr/local/bin/historic_sys"]
