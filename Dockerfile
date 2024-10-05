# # Fase 1: Construção
# FROM rust:1.81 as builder

# # Criação de um diretório para o projeto e configuração do contexto de trabalho
# WORKDIR /app

# # Copiar Cargo.toml para que as dependências sejam compiladas
# COPY Cargo.toml ./

# # Compilar as dependências para cachear dependências
# RUN cargo build --release || true

# # Copiar o código-fonte do projeto
# COPY ./src ./src

# # Construir o binário final
# RUN cargo build --release

# # Fase 2: Imagem final
# FROM debian:bookworm-slim

# # Instalar as bibliotecas necessárias
# RUN apt-get update && apt-get install -y \
#     libssl-dev \
#     ca-certificates \
#     && apt-get clean && rm -rf /var/lib/apt/lists/*

# # Copiar o binário da fase de construção
# COPY --from=builder /app/target/release/historic_sys /usr/local/bin/historic_sys

# # Definir o ponto de entrada do contêiner
# CMD ["historic_sys"]










FROM rust:1.80 as build

RUN USER=root cargo new --bin historic_sys
WORKDIR /historic_sys

# COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/historic_sys*
RUN cargo build --release

FROM debian:bookworm-slim

# Instalar as bibliotecas necessárias
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=build /historic_sys/target/release/historic_sys .

CMD ["./historic_sys"]