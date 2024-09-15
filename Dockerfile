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