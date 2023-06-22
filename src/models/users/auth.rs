use serde::Deserialize;
use validator::Validate;

use crate::{
    models::users::{role::Role, User},
    utils::{regex::REGEX_USERNAME, validation::check_password_strength},
};

#[derive(Debug, Deserialize, Default, Validate)]
#[serde(rename_all = "camelCase", default)]
pub struct Registrar {
    #[validate(email(message = "Please provide a valid email address"))]
    pub email: String,
    #[validate(regex(
        path = "REGEX_USERNAME",
        message = "The username must be 5-16 characters long and start with a letter, and can only contain letters, numbers, and underscores"
    ))]
    username: String,
    #[validate(custom(
        function = "check_password_strength",
        message = "The password must be 8-16 characters long and contain at least one uppercase letter, one lowercase letter, and one number"
    ))]
    password: String,
    #[validate(must_match(other = "password", message = "The passwords do not match"))]
    password_confirm: String,
    #[validate(length(
        min = 6,
        max = 6,
        message = "Invalid registration code, please check your email and try again"
    ))]
    pub code: String,
}

impl Registrar {
    pub fn build(self) -> User {
        User::new(self.email, self.username, self.password, Role::User)
    }
}
