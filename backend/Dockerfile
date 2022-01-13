# Adapted from: https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
# This Dockerfile must be built from the parent directory.

ARG BIN_NAME

FROM rust as build

ARG BIN_NAME

COPY ./config.toml /config.toml

WORKDIR /backend

RUN USER=root cargo new --bin $BIN_NAME
RUN USER=root cargo new --lib apheleia
RUN USER=root cargo new --lib apheleia-proc

COPY ./backend/Cargo.toml ./Cargo.toml
COPY ./backend/Cargo.lock ./Cargo.lock

COPY ./backend/$BIN_NAME/Cargo.toml ./$BIN_NAME/Cargo.toml
COPY ./backend/apheleia/Cargo.toml ./apheleia/Cargo.toml
COPY ./backend/apheleia-proc/Cargo.toml ./apheleia-proc/Cargo.toml

# cache dependencies
RUN cargo build --release

# delete dummy binaries
RUN rm ./target/release/deps/*$BIN_NAME*
RUN rm ./target/release/deps/*apheleia*
# copy source tree
RUN rm ./$BIN_NAME/src/*.rs
COPY ./backend/$BIN_NAME/src ./$BIN_NAME/src
RUN rm ./apheleia/src/*.rs
COPY ./backend/apheleia/src ./apheleia/src
RUN rm ./apheleia-proc/src/*.rs
COPY ./backend/apheleia-proc/src ./apheleia-proc/src

WORKDIR $BIN_NAME

RUN cargo build --release

# final base
FROM debian:buster-slim

ARG BIN_NAME

# copy build artifact from build container
COPY --from=build /backend/target/release/$BIN_NAME .

CMD ./$BIN_NAME
