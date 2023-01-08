# rust-api-example
An example API using Rust and Postgres.

- The database framework is [Diesel](https://diesel.rs/).
- The API framework is [Rocket](https://rocket.rs/).

## How to use

### Prerequisites
- Docker
- [Rustup](https://www.rust-lang.org/tools/install)
NB: If you don't have Rust installed and don't want to install it, remove the commented out section
from `docker-compose.yml` and `.env`.  However, currently the file watch functionality isn't working
correctly on Windows OS.

### Running it

1. `docker compose up`
2. `cargo watch -x run` (or `cargo run` if you don't have cargo-watch installed).

If using the rust-api-server docker container you can skip Step 2.

### Trying it

When the application runs, it will run the database migrations to create a users table and seed it with
some random example data.

Try accessing a GET endpoint `http://localhost:8000/users` or `http://localhost:8000/version`.
NB: The API endpoints are configured so the content-type on the request must be set to `application/json`.
