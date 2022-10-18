pub mod mongo_repo;

#[derive(Debug)]
pub enum DbError {
    IdParseError,
    NotFound,
    InternalError(mongodb::error::Error)
}