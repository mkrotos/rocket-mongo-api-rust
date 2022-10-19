use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::domain::{custom_error::CustomError, user_model::User};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

    pub fn into_new_user(self: Self) -> User {
        User {
            id: None,
            name: self.name,
            location: self.location,
            title: self.title,
        }
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

#[cfg(test)]
mod tests {
    use mongodb::bson::oid::ObjectId;

    use crate::domain::user_model::User;

    use super::UserDto;

    #[test]
    fn map_into_new_user() {
        let dto = create_dto(None);

        let expected = create_user(None);

        assert_eq!(dto.into_new_user(), expected);
    }
    #[test]
    fn map_into_new_user_when_dto_have_id() {
        let dto = create_dto(Some("some id".to_string()));
        let expected = create_user(None);

        assert_eq!(dto.into_new_user(), expected);
    }

    #[test]
    fn map_user_to_dto() {
        let id = "634fb14ef4a83fee4418fc83".to_string();
        let obj_id = ObjectId::parse_str(&id).expect("This should be a correct id");
        let user = create_user(Some(obj_id));
        let expected_dto = create_dto(Some(id));

        let actual_dto: UserDto = user.into();
        assert_eq!(actual_dto, expected_dto);
    }

    fn create_dto(id: Option<String>) -> UserDto {
        UserDto {
            id,
            title: "Title".to_string(),
            location: "Location".to_string(),
            name: "Name".to_string(),
        }
    }

    fn create_user(id: Option<ObjectId>) -> User {
        User {
            id,
            title: "Title".to_string(),
            location: "Location".to_string(),
            name: "Name".to_string(),
        }
    }
}
