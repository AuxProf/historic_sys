
FROM rust:1.80 as build

RUN USER=root cargo new --bin historic_sys
WORKDIR /historic_sys

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/historic_sys*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /historic_sys/target/release/historic_sys .

CMD ["./historic_sys"]