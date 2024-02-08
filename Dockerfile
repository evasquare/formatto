# syntax=docker/dockerfile:1

ARG NODE_VERSION=20.11.0
FROM node:${NODE_VERSION}-buster


WORKDIR /usr/src/app
ENV RUST_VERSION=${RUST_VERSION}


# Set timezone (UTC)
RUN ln -sf /usr/share/zoneinfo/Etc/UTC /etc/localtime
# Use UTF-8
RUN locale-gen C.UTF-8 || true
ENV LANG=C.UTF-8


# Install dependencies
RUN apt-get update && apt-get install -y curl build-essential

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"


# Copy the rest of the source files into the image.
USER node
COPY . .

# Install code dependencies.
USER root
RUN npm i
RUN cd ./wasm && cargo check

# Build the plugin.
CMD npm run build \
    mv ./main.js ./build/main.js \
    mv ./styles.css ./build/styles.css \
    mv ./manifest.json ./build/manifest.json