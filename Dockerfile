FROM rust:latest as build

RUN curl https://gist.githubusercontent.com/vincent/4ce2f9f37b1ac846f84c/raw/d9f0977a0db29eefade49ebf1ea4b7e1498d55b2/moveAlong.js
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