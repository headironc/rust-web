use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // E11000 duplicate key error collection: hammer.user index: userId dup key: { userId: \"1111111\" }
    pub static ref REGEX_DUPLICATE_KEY: Regex = Regex::new(r#"collection: (\w+)\.(\w+)\b.* dup key: \{ (\w+): "([^"]+)" \}"#).unwrap();
    pub static ref REGEX_USERNAME: Regex =
        Regex::new(r"^[a-zA-Z][a-zA-Z0-9_]{4,15}$").unwrap();
    // integer, can't start with 0, 1-9位
    pub static ref REGEX_USER_ID: Regex = Regex::new(r"^[1-9]\d{0,8}$").unwrap();
}
