FROM rust:1.66

WORKDIR /api_server

RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres

EXPOSE 8000

VOLUME ["/usr/local/cargo"]