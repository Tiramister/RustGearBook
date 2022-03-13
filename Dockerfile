FROM rust:1.59

# apt
RUN apt-get update
RUN apt-get install -y sqlite3
RUN rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*

# rust
RUN rustup component add rustfmt clippy rust-src
RUN cargo install cargo-edit

# build inside the container
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

# keep running
CMD [ "/bin/sh", "-c", "while sleep 1000; do :; done" ]
