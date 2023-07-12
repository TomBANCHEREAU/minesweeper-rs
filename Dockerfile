FROM rust:bookworm as clientBuilder

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /build
COPY core core
COPY client client

WORKDIR /build/client
RUN wasm-pack build --release --target web


FROM rust:bookworm as serverBuilder

WORKDIR /build
COPY core core
COPY server server

WORKDIR /build/server
RUN cargo build --release

FROM debian:bookworm

COPY --from=clientBuilder /build/client/pkg client/pkg
COPY --from=clientBuilder /build/client/images client/images
COPY --from=clientBuilder /build/client/index.html client/index.html
COPY --from=clientBuilder /build/client/lobby.html client/lobby.html

COPY --from=serverBuilder /build/server/target/release/server server/server

EXPOSE 8080

CMD [ "server/server" ]

