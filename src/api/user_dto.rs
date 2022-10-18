use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::domain::{custom_error::CustomError, user_model::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: Option<String>,
    pub name: String,
    pub location: String,
    pub title: String,
}

impl UserDto {
    pub fn into_user_with_id(self: Self, id: &str) -> Result<User, CustomError> {
        Ok(User {
            id: Some(ObjectId::parse_str(id)?),
            name: self.name,
            location: self.location,
            title: self.title,
        })
    }

    pub fn into_new_user(self: Self) -> Result<User, CustomError> {
        Ok(User {
            id: None,
            name: self.name,
            location: self.location,
            title: self.title,
        })
    }
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id.map(|obj| obj.to_string()),
            name: user.name,
            location: user.location,
            title: user.title,
        }
    }
}
