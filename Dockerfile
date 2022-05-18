# Adapted from: https://dev.to/rogertorres/first-steps-with-docker-rust-30oi

# So that matching GLIBC version is used by debian:buster-slim image
FROM rust:slim-buster as build

WORKDIR /build

# create dummy packages
RUN USER=root cargo new --bin bin
RUN USER=root cargo new --lib apheleia

# copy over manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

COPY ./bin/Cargo.toml ./bin/Cargo.toml
COPY ./apheleia/Cargo.toml ./apheleia/Cargo.toml

# cache dependencies
RUN cargo build --release

# delete dummy binaries
RUN rm ./target/release/deps/bin*
RUN rm ./target/release/deps/*apheleia*

# copy source tree
RUN rm ./bin/src/*.rs
COPY ./bin/src ./bin/src
RUN rm ./apheleia/src/*.rs
COPY ./apheleia/src ./apheleia/src

COPY ./apheleia/migrations ./apheleia/migrations

RUN apt-get update
RUN apt-get install libpq-dev -y

WORKDIR bin

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update
RUN apt-get install libpq-dev -y

# copy build artifact from build container
COPY --from=build /build/target/release/bin /app

CMD ["/app", "sync"]
