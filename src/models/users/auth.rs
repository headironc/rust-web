use serde::Deserialize;
use validator::Validate;

use crate::{
    models::users::{role::Role::Staff, User},
    utils::regex::{REGEX_USERNAME, REGEX_USER_ID},
};

#[derive(Debug, Deserialize, Default, Validate)]
#[serde(rename_all = "camelCase", default)]
pub struct Registrar {
    #[validate(regex(
        path = "REGEX_USERNAME",
        message = "The username must be 5-16 characters long and start with a letter, and can only contain letters, numbers, and underscores."
    ))]
    username: String,
    #[validate(regex(
        path = "REGEX_USER_ID",
        message = "The user_id must be 1-9 digits long and can't start with 0."
    ))]
    user_id: String,
    password: String,
    password_confirm: String,
}

impl Registrar {
    pub fn build(self) -> User {
        User::new(self.username, self.user_id, self.password, Staff)
    }
}
