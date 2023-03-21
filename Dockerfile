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

FROM gcr.io/distroless/cc-debian10

RUN apt install -y openssl

COPY --from=build /usr/src/boats/boats_api/target/release/boats_api /usr/local/bin/backend
COPY --from=build /usr/src/boats/boats_web/dist /usr/local/bin/dist

WORKDIR /usr/local/bin
CMD ["backend"]