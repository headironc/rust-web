use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Default, Validate)]
#[serde(default)]
pub struct MailValidator {
    #[validate(email(message = "Please provide a valid email address"))]
    pub email: String,
}
