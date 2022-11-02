FROM rust:1.64

WORKDIR /usr/src/rust-terminal-app
COPY . .

RUN cargo install --path .

RUN cargo build
