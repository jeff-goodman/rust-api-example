FROM rust:1.66

# RUN apt-get install -y libpq-dev

# ARG database_url
# ENV DATABASE_URL ${database_url}
# RUN echo ${DATABASE_URL}

WORKDIR /api_server

RUN cargo install cargo-watch
# RUN cargo install diesel_cli --no-default-features --features postgres
# RUN diesel setup
# RUN diesel migration run

EXPOSE 8000

ENTRYPOINT cargo watch -x run