# builder image
FROM rust:1.59 as builder

WORKDIR /app

# pre-build Cargo.toml
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/todo_list*

# build
COPY ./src ./src
COPY ./templates ./templates
RUN cargo build --release

# release image
FROM debian:latest

# install SQLite3
RUN apt-get update
RUN apt-get install -y sqlite3
RUN rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*

# copy binary and run
COPY --from=builder /app/target/release/todo-list /usr/local/bin/todo-list
CMD ["todo-list"]
