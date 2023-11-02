FROM rust:bookworm as ClientBuilder

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /build
COPY core core
COPY model model
COPY client client

WORKDIR /build/client
RUN trunk build --release

FROM rust:bookworm as ServerBuilder

WORKDIR /build
COPY core core
COPY model model
COPY server server

WORKDIR /build/server
RUN cargo build --release

FROM alpine as Runner

COPY --from=ClientBuilder /build/client/dist client/dist
COPY --from=ClientBuilder /build/client/images client/images

COPY  --from=ServerBuilder /build/server/target/release/server server/server

EXPOSE 9000

CMD [ "server/server" ]
