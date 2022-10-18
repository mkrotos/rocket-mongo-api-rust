use rocket::http::Status;

use crate::domain::custom_error::CustomError;


pub mod user_api;
pub mod user_dto;

impl From<CustomError> for Status {
    fn from(error: CustomError) -> Self {
        match error {
            CustomError::IdParseError => Status::BadRequest,
            CustomError::NotFound => Status::NotFound,
            CustomError::DbError(reason) => {
                println!("Internal db error: {}", reason);
                Status::InternalServerError
            }
        }
    }
}
