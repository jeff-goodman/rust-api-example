#[macro_use] extern crate rocket;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::serde::json::{Value, json};

mod routes;
mod shared;
use routes::{version, users};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[launch]
fn rocket() -> _ {
    let connection = &mut shared::connection::establish_connection();
    connection.run_pending_migrations(MIGRATIONS);

    rocket::build()
        .attach(version::stage())
        .attach(users::stage())
        .register("/", catchers![not_found])
}