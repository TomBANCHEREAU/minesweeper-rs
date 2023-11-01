FROM debian:bookworm

COPY ./client/dist client/dist
COPY ./client/images client/images

COPY ./target/release/server server/server

EXPOSE 9000

CMD [ "server/server" ]
