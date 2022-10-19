use rocket::{http::Status, serde::json::Json, State};

use crate::repository::RepoAdapter;

use super::user_dto::UserDto;

#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<RepoAdapter>,
    new_user: Json<UserDto>,
) -> Result<Json<String>, Status> {
    let id = db.create_user(new_user.into_inner().into_new_user())?;
    Ok(Json(id))
}

#[get("/user/<id>")]
pub fn get_user(db: &State<RepoAdapter>, id: String) -> Result<Json<UserDto>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user = db.get_user(&id)?;
    Ok(Json(user.into()))
}

#[put("/user/<id>", data = "<new_user>")]
pub fn update_user(
    db: &State<RepoAdapter>,
    id: String,
    new_user: Json<UserDto>,
) -> Result<Json<UserDto>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };

    let data = new_user.into_inner().into_user_with_id(&id)?;
    let updated_user = db.update_and_get_user(&id, data)?;
    Ok(Json(updated_user.into()))
}

#[delete("/user/<id>")]
pub fn delete_user(db: &State<RepoAdapter>, id: String) -> Result<Json<&str>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    db.delete_user(&id)?;
    Ok(Json("User successfully deleted"))
}

#[get("/users")]
pub fn get_all_users(db: &State<RepoAdapter>) -> Result<Json<Vec<UserDto>>, Status> {
    db.get_all_users()
        .map(|user_list| Json(user_list.into_iter().map(|user| user.into()).collect()))
        .map_err(|err| err.into())
}
