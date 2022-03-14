FROM rust:1.59

# rust and cargo toolchains
RUN rustup component add rustfmt clippy rust-src
RUN cargo install cargo-edit

# build inside the container
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

# install npm
RUN apt-get update  \
    && apt-get install -y \
    npm \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# install wasm-pack and build
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# keep running
CMD [ "/bin/sh", "-c", "while sleep 1000; do :; done" ]
