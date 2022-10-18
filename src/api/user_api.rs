use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

use crate::{domain::user_model::User, repository::mongo_repo::MongoRepo};

#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let user = db.create_user(data)?;
    Ok(Json(user))
}

#[get("/user/<id>")]
pub fn get_user(db: &State<MongoRepo>, id: String) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user = db.get_user(&id)?;
    Ok(Json(user))
}

#[put("/user/<id>", data = "<new_user>")]
pub fn update_user(
    db: &State<MongoRepo>,
    id: String,
    new_user: Json<User>,
) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let data = User {
        id: Some(ObjectId::parse_str(&id).map_err(|_| Status::BadRequest)?),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    db.update_user(&id, data)?;
    let updated_user = db.get_user(&id)?;
    Ok(Json(updated_user))
}

#[delete("/user/<id>")]
pub fn delete_user(db: &State<MongoRepo>, id: String) -> Result<Json<&str>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_user(&id)?;
    if result.deleted_count == 1 {
        Ok(Json("User successfully deleted"))
    } else {
        Err(Status::NotFound)
    }
}

#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<User>>, Status> {
    db.get_all_users()
        .map(|user| Json(user))
        .map_err(|err| err.into())
}
