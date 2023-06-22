use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Root,
    Admin,
    Author,
    User,
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.as_str() {
            "root" => Ok(Self::Root),
            "admin" => Ok(Self::Admin),
            "author" => Ok(Self::Author),
            "user" => Ok(Self::User),
            _ => Err(serde::de::Error::custom(format!(
                "未知的角色: {}，应为 root, admin, author, user",
                s
            ))),
        }
    }
}
