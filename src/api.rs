use rocket::http::Status;

use crate::repository::DbError;

pub mod user_api;

impl From<DbError> for Status {
    fn from(error: DbError) -> Self {
        match error {
            DbError::IdParseError => Status::BadRequest,
            DbError::NotFound => Status::NotFound,
            DbError::InternalError(reason) => {
                println!("Internal db error: {}", reason);
                Status::InternalServerError
            }
        }
    }
}
