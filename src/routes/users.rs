use diesel::prelude::*;
use rocket::serde::json::Json;

use crate::shared::{models, schema, connection};
use connection::establish_connection;
use schema::users::dsl::*;
use models::User;


#[get("/<user_id>", format = "application/json")]
fn get(user_id: String) -> Json<Vec<User>> {
    let connection = &mut establish_connection();
    let user_vec = users
        .filter(id.eq(user_id))
        .limit(1)
        .load::<User>(connection)
        .expect("Error loading user");

    Json(user_vec)
}

#[get("/", format = "application/json")]
fn get_all() -> Json<Vec<User>> {
    let connection = &mut establish_connection();
    let user_vec = users
        .load::<User>(connection)
        .expect("Error loading users");

    Json(user_vec)
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("users", |rocket| async {
        rocket.mount("/users", routes![get, get_all])
    })
}