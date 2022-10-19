use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

use crate::repository::mongo_repo::MongoRepo;

use super::user_dto::UserDto;

#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    new_user: Json<UserDto>,
) -> Result<Json<InsertOneResult>, Status> {
    let user = db.create_user(new_user.into_inner().into_new_user())?;
    Ok(Json(user))
}

#[get("/user/<id>")]
pub fn get_user(db: &State<MongoRepo>, id: String) -> Result<Json<UserDto>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user = db.get_user(&id)?;
    Ok(Json(user.into()))
}

#[put("/user/<id>", data = "<new_user>")]
pub fn update_user(
    db: &State<MongoRepo>,
    id: String,
    new_user: Json<UserDto>,
) -> Result<Json<UserDto>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let data = new_user.into_inner().into_user_with_id(&id)?;
    db.update_user(&id, data)?;
    let updated_user = db.get_user(&id)?;
    Ok(Json(updated_user.into()))
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
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<UserDto>>, Status> {
    db.get_all_users()
        .map(|user_list| Json(user_list.into_iter().map(|user| user.into()).collect()))
        .map_err(|err| err.into())
}
