use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

use crate::domain::{custom_error::CustomError, user_model::User};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub fn init() -> Result<Self, Error> {
        dotenv().ok();
        let uri = env::var("MONGOURI").expect("Mongo connection string have NOT been set");
        let client = Client::with_uri_str(uri)?;
        let db = client.database("RustDB");
        db.run_command(doc! {"ping": 1}, None)?;
        println!("Pinged db successfully");
        let col: Collection<User> = db.collection("Users");

        println!("Successfully connected to MongoDB.");
        Ok(MongoRepo { col })
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, CustomError> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };

        self.col.insert_one(new_doc, None).map_err(|err| err.into())
    }

    pub fn get_user(&self, id: &str) -> Result<User, CustomError> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let user = self.col.find_one(filter, None)?;
        user.ok_or(CustomError::NotFound)
    }

    pub fn update_user(&self, id: &str, new_user: User) -> Result<UpdateResult, CustomError> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "id": new_user.id,
                "name": new_user.name,
                "location": new_user.location,
                "title": new_user.title,
            }
        };
        self.col
            .update_one(filter, new_doc, None)
            .map_err(|err| err.into())
    }

    pub fn delete_user(&self, id: &str) -> Result<DeleteResult, CustomError> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let user_detail = self.col.delete_one(filter, None)?;
        Ok(user_detail)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, CustomError> {
        let cursor = self.col.find(None, None)?;
        cursor
            .map(|it| it.map_err(|err| err.into()))
            .into_iter()
            .collect()
    }
}

impl From<Error> for CustomError {
    fn from(err: Error) -> Self {
        CustomError::DbError(err)
    }
}
