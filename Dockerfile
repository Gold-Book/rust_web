FROM rust:1.39

RUN cargo install systemfd cargo-watch

WORKDIR /usr/src/example

COPY src /usr/src/example/src
COPY Cargo.toml /usr/src/example/

RUN cargo fetch
RUN cargo build

COPY ./entrypoint.sh .
RUN chmod 755 ./entrypoint.sh

EXPOSE 3000
CMD ["./entrypoint.sh"]
