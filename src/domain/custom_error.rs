use mongodb::bson::oid;

#[derive(Debug)]
pub enum CustomError {
    IdParseError,
    NotFound,
    DbError(mongodb::error::Error),
}


impl From<oid::Error> for CustomError {
    fn from(_: oid::Error) -> Self {
        CustomError::IdParseError
    }
}