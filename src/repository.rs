use crate::domain::{custom_error::CustomError, user_model::User};

use self::mongo_repo::MongoRepo;
use dotenv::dotenv;
use mongodb::results::InsertOneResult;
use std::env;

mod mongo_repo;

pub struct RepoAdapter {
    repo: MongoRepo,
}

impl RepoAdapter {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").expect("Mongo connection string have NOT been set");
        let repo = MongoRepo::init(uri).expect("Failed to connect to mongo db");
        Self { repo }
    }

    pub fn create_user(&self, new_user: User) -> Result<String, CustomError> {
        let insert_result_to_id = |it: InsertOneResult| {
            it.inserted_id
                .as_object_id()
                .map(|id| id.to_string())
                .ok_or_else(|| CustomError::IdParseError)
        };

        self.repo.create_user(new_user).map(insert_result_to_id)?
    }
    pub fn get_user(&self, id: &str) -> Result<User, CustomError> {
        self.repo.get_user(id)
    }
    pub fn update_and_get_user(&self, id: &str, new_user: User) -> Result<User, CustomError> {
        self.repo.update_user(id, new_user)?;
        self.repo.get_user(&id)
    }
    pub fn delete_user(&self, id: &str) -> Result<(), CustomError> {
        let result = self.repo.delete_user(id)?;
        if result.deleted_count == 1 {
            Ok(())
        } else {
            Err(CustomError::NotFound)
        }
    }
    pub fn get_all_users(&self) -> Result<Vec<User>, CustomError> {
        self.repo.get_all_users()
    }
}
