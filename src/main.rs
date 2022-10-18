#[macro_use]
extern crate rocket;
use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users};
use repository::mongo_repo::MongoRepo;
use rocket::{get, http::Status, serde::json::Json};

mod api;
mod domain;
mod repository;

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json("Hello from rust and mongo".to_string()))
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init().expect("Failed to connect to mongo db");
    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
}
