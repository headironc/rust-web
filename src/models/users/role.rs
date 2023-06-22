use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Root,
    Admin,
    Staff,
}

impl Default for Role {
    fn default() -> Self {
        Self::Staff
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
            "staff" => Ok(Self::Staff),
            _ => Err(serde::de::Error::custom(format!(
                "未知的角色: {}，应为 root, admin, staff 中的一个",
                s
            ))),
        }
    }
}
