# builder container
FROM rust:1.59 as builder

# install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# build wasm files
WORKDIR /app
COPY ./src ./src
COPY Cargo.toml .
RUN wasm-pack build


# release container
FROM node:17-alpine as release

# see https://github.com/webpack/webpack/issues/14532
ENV NODE_OPTIONS=--openssl-legacy-provider

# run app
WORKDIR /app
COPY --from=builder /app/pkg ./pkg
COPY ./www ./www
WORKDIR /app/www
CMD [ "npm", "run", "start" ]
