# rust-api-example
An example API using Rust and Postgres based on a combination of a few different web tutorials and framework documentation.

- The database framework is [Diesel](https://diesel.rs/).
- The API framework is [Rocket](https://rocket.rs/).
- File watcher reloading using [cargo-watch](https://crates.io/crates/cargo-watch).

## How to use

### Prerequisites
- Docker

This example does not require Rust or Postgres to be installed locally. 

The `cargo watch` command that runs when the container starts works best on Linux (not Windows) if you want it to detect file changes.

If you are using WSL, clone this repo on the WSL drive rather than a mounted drive to get automatic reloading on file changes.

### Running it

1. `docker compose up`
2. Sit back and watch it compile ☕

### Trying it

When the application runs it will: 
- Create the database
- Run the database migrations to create a users table
- Seed the users table with some random example data.

Try accessing a GET endpoint `http://localhost:8000/users` or `http://localhost:8000/version`.
NB: The API endpoints are configured so the content-type on the request must be set to `application/json`.
