FROM ghcr.io/plc-lang/rust-llvm:latest as build

WORKDIR /usr/src/boats
COPY . .

ENV DEBIAN_FRONTEND=noninteractive
RUN apt update
RUN apt install -y pkg-config librust-openssl-sys-dev

# LIBCLANG_PATH
#ENV LIBCLANG_PATH /usr/src/boats/clang_16/lib

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

# RUN ls $LIBCLANG_PATH

RUN cd boats_web && trunk build --release
RUN cd boats_api && cargo build --release

FROM bookworm

RUN apt install -y openssl libssl-dev libssl1.1

# Get Rust
ARG RUST_VERSION=1.67.0
RUN curl https://sh.rustup.rs -sSf | bash -s -- --profile minimal --default-toolchain none -y
ENV PATH="${CARGO_HOME}/bin:${PATH}"

RUN rustup toolchain install nightly  \
                # && rustup toolchain install $RUST_VERSION \
                # && rustup default $RUST_VERSION \
                # && rustup component add clippy rustfmt llvm-tools-preview  \
                # && rustup component add --toolchain nightly llvm-tools-preview  \
                # && rustup target add aarch64-unknown-linux-gnu \
                # && rustup target add x86_64-unknown-linux-musl \
                # && rustup target add wasm32-unknown-unknown
                && echo "rust deps installed"

COPY --from=build /usr/src/boats/boats_api/target/release/boats_api /usr/local/bin/backend
COPY --from=build /usr/src/boats/boats_web/dist /usr/local/bin/dist

COPY nginx.site.conf /etc/nginx/sites-enabled/boats
RUN nginx -t

RUN /usr/local/bin/backend
CMD ["nginx"]