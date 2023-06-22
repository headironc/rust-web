use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{
    database::{Collection::Users, Database},
    errors::Error,
    models::IntoJson,
};

pub mod auth;
pub mod codes;
pub mod mail_validator;
pub mod role;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    email: String,
    username: String,

    password: String,
    role: role::Role,
    created_at: DateTime,
    updated_at: DateTime,
}

impl User {
    pub fn new(email: String, username: String, password: String, role: role::Role) -> Self {
        Self {
            id: ObjectId::new(),
            email,
            username,
            password: Self::hash_password(password),
            role,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }

    fn hash_password(password: String) -> String {
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        argon2
            .hash_password(password.as_ref(), &salt)
            .unwrap()
            .to_string()
    }

    pub fn verify_password(&self, candidate_password: String) -> bool {
        let argon2 = Argon2::default();

        let password_hash = PasswordHash::new(self.password.as_str()).unwrap();

        argon2
            .verify_password(candidate_password.as_ref(), &password_hash)
            .is_ok()
    }

    pub async fn create(user: &Self, db: &Database) -> Result<(), Error> {
        db.collection::<Self>(Users).insert_one(user, None).await?;

        Ok(())
    }

    pub async fn find_one_by_email(email: String, db: &Database) -> Result<Option<Self>, Error> {
        let option = db
            .collection::<Self>(Users)
            .find_one(doc! { "email": email }, None)
            .await?;

        Ok(option)
    }
}

impl IntoJson for User {
    fn into_json(self) -> Value {
        let created_at = self.created_at.try_to_rfc3339_string().unwrap();
        let updated_at = self.updated_at.try_to_rfc3339_string().unwrap();

        json!({
            "id": self.id.to_hex(),
            "email": self.email,
            "username": self.username,
            "role": self.role,
            "createdAt": created_at,
            "updatedAt": updated_at,
        })
    }
}
