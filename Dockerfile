FROM rust:latest as build

RUN curl -SL https://github.com/llvm/llvm-project/releases/download/llvmorg-16.0.0/clang+llvm-16.0.0-x86_64-linux-gnu-ubuntu-18.04.tar.xz \
 | tar -xJC . && \
 mv clang+llvm-10.0.0-x86_64-linux-gnu-ubuntu-18.04 clang_10 && \
 echo 'export PATH=/clang_10/bin:$PATH' >> ~/.bashrc && \
 echo 'export LIBCLANG_PATH=/clang_10/bin' >> ~/.bashrc && \
 echo 'export LD_LIBRARY_PATH=/clang_10/lib:$LD_LIBRARY_PATH' >> ~/.bashrc
RUN source ~/.bashrc

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/boats
COPY . .

RUN cd boats_web && trunk build --release
RUN cd boats_api && cargo build --release

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/src/boats/boats_api/target/release/boats_api /usr/local/bin/backend
COPY --from=build /usr/src/boats/boats_web/dist /usr/local/bin/dist

WORKDIR /usr/local/bin
CMD ["backend"]