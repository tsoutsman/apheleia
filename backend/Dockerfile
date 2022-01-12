# Adapted from: https://dev.to/rogertorres/first-steps-with-docker-rust-30oi
# This Dockerfile must be built from the parent directory.

FROM rust as build

COPY ./config.toml /config.toml

WORKDIR /backend

RUN USER=root cargo new --bin apheleia
RUN USER=root cargo new --lib apheleia-proc

COPY ./backend/Cargo.toml ./Cargo.toml
COPY ./backend/Cargo.lock ./Cargo.lock

COPY ./backend/apheleia/Cargo.toml ./apheleia/Cargo.toml
COPY ./backend/apheleia-proc/Cargo.toml ./apheleia-proc/Cargo.toml

# cache dependencies
RUN cargo build --release

# delete dummy binaries
RUN rm ./target/release/deps/*apheleia*
# copy source tree
RUN rm ./apheleia/src/*.rs
COPY ./backend/apheleia/src ./apheleia/src
RUN rm ./apheleia-proc/src/*.rs
COPY ./backend/apheleia-proc/src ./apheleia-proc/src

WORKDIR apheleia

RUN cargo build --release

ARG bin_name
RUN echo (*(N[1])) > __var1

# final base
FROM debian:buster-slim
# copy build artifact from build container
COPY --from=build /backend/target/release/$bin_name .

CMD ["./sbhs"]
