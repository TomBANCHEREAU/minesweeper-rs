FROM rust:bookworm as builder

RUN cargo install cargo-make trunk
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN rustup target add wasm32-unknown-unknown

WORKDIR /build
COPY core core
COPY model model
COPY client client
COPY server server
COPY Cargo.toml .

RUN cargo make build

FROM debian:bookworm as runner

COPY --from=builder /build/client/dist client/dist
COPY --from=builder /build/client/images client/images

COPY --from=builder /build/target/release/server server/server

EXPOSE 9000

CMD [ "server/server" ]
