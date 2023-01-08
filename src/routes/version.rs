use chrono::prelude::*;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Version {
    version: u8,
    date_local: String,
    date_utc: String,
}

#[get("/", format = "application/json")]
fn get() -> Json<Version> {
    let date_local = Local::now().to_rfc3339();
    let date_utc = Utc::now().to_rfc3339();
    Json(Version { version: 1, date_local, date_utc })
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("version", |rocket| async {
        rocket.mount("/version", routes![get])
    })
}