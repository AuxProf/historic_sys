# Fase de build
FROM rust:1.80 as builder

WORKDIR /app

# Copiar os arquivos do cargo.toml e o lock para o cache das dependências
COPY Cargo.toml Cargo.lock ./

# Primeiro compilamos as dependências para que possamos aproveitar o cache do Docker
RUN cargo fetch
RUN cargo build --release --bin dummy || true

# Agora copiamos o código fonte completo e construímos novamente
COPY ./src ./src

RUN cargo build --release

# Fase de execução
FROM debian:bookworm-slim

# Instalar as bibliotecas necessárias
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Copiar o binário compilado da fase de build
COPY --from=builder /app/target/release/historic_sys /usr/local/bin/historic_sys

# Executar o binário
CMD ["/usr/local/bin/historic_sys"]
