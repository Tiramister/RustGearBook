## builder container
FROM rust:1.59 as builder

# install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# install node
ENV NODE_VERSION=16.14.0
ENV NVM_DIR=/root/.nvm
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash \
    && . "${NVM_DIR}/nvm.sh" \
    && nvm install ${NODE_VERSION} \
    && nvm use ${NODE_VERSION} \
    && nvm alias default v${NODE_VERSION}
ENV PATH="${NVM_DIR}/versions/node/v${NODE_VERSION}/bin/:${PATH}"

# build wasm files
WORKDIR /app
COPY ./src ./src
COPY Cargo.toml .
RUN wasm-pack build

# bundle by webpack
COPY ./www ./www
WORKDIR /app/www
RUN npm run build


## release container
FROM nginx:1.21.6 as release

# deploy with nginx
COPY --from=builder /app/www/dist /dist
COPY ./nginx/default.conf /etc/nginx/conf.d/default.conf
RUN service nginx restart
