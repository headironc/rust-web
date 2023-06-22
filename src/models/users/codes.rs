use mongodb::bson::{doc, oid::ObjectId, Bson, DateTime};
use serde::{Deserialize, Serialize};

use crate::{
    database::{Collection::Codes, Database},
    errors::Error::{self, InternalServerError},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Code {
    #[serde(rename = "_id")]
    id: ObjectId,
    email: String,
    code: String,
    code_type: CodeType,
    active: bool,
    created_at: DateTime,
    expired_at: DateTime,
}

impl Code {
    /// Create a new code instance, with a valid time in minutes, unit is minute
    pub fn new(email: String, code: String, code_type: CodeType, code_expire: i64) -> Self {
        let now = DateTime::now().timestamp_millis();
        let expired_at = now + code_expire * 60 * 1000;

        Self {
            id: ObjectId::new(),
            email,
            code,
            code_type,
            active: true,
            created_at: DateTime::now(),
            // 默认当前时间加上15分钟
            expired_at: DateTime::from_millis(expired_at),
        }
    }

    pub fn is_expired(&self) -> bool {
        DateTime::now() > self.expired_at
    }

    pub fn is_valid(&self, candidate: String) -> bool {
        self.active && !self.is_expired() && self.code == candidate
    }

    pub async fn create(code: Self, db: &Database) -> Result<(), Error> {
        db.collection::<Self>(Codes).insert_one(code, None).await?;

        Ok(())
    }

    /// Find one code instance by email
    /// code_type is the type of code, like registration, etc.
    /// active means the code is not used or expired
    pub async fn find_one_by_email(
        email: String,
        code_type: CodeType,
        db: &Database,
    ) -> Result<Option<Self>, Error> {
        let option = db
            .collection::<Self>(Codes)
            .find_one(
                doc! {
                    "email": email,
                    "codeType": code_type,
                    "active": true
                },
                None,
            )
            .await?;

        Ok(option)
    }

    pub async fn deactivate_by_id(&self, db: &Database) -> Result<(), Error> {
        let option = db
            .collection::<Self>(Codes)
            .find_one_and_update(
                doc! { "_id": self.id },
                doc! { "$set": { "active": false } },
                None,
            )
            .await?;

        if option.is_none() {
            return Err(InternalServerError(format!(
                "Failed to deactivate code with id {}",
                self.id.to_hex()
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CodeType {
    Registration,
}

impl From<CodeType> for Bson {
    fn from(code_type: CodeType) -> Self {
        use CodeType::*;

        match code_type {
            Registration => Bson::String("registration".to_owned()),
        }
    }
}
