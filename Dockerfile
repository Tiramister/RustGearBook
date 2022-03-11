FROM rust:1.59
RUN rustup component add rustfmt
RUN rustup component add clippy
WORKDIR /work
