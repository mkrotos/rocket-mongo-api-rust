#[macro_use]
extern crate rocket;
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use repository::RepoAdapter;
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
    let db = RepoAdapter::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![hello])
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])
}
