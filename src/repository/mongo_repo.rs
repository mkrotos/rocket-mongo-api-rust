use std::env;
extern crate dotenv;
use dotenv::dotenv;
use mongodb::{
    bson::{
        doc,
        oid::{self, ObjectId},
    },
    error::Error,
    results::{InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

use crate::domain::user_model::User;

use super::DbError;

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

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, DbError> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };

        self.col.insert_one(new_doc, None).map_err(|err| err.into())
    }

    pub fn get_user(&self, id: &str) -> Result<User, DbError> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! {"_id": obj_id};
        let user = self.col.find_one(filter, None)?;
        user.ok_or(DbError::NotFound)
    }

    pub fn update_user(&self, id: &str, new_user: User) -> Result<UpdateResult, DbError> {
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
}

impl From<Error> for DbError {
    fn from(err: Error) -> Self {
        DbError::InternalError(err)
    }
}

impl From<oid::Error> for DbError {
    fn from(_: oid::Error) -> Self {
        DbError::IdParseError
    }
}
