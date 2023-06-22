use validator::ValidationError;

pub fn check_password_strength(password: &str) -> Result<(), ValidationError> {
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;

    if password.len() < 8 || password.len() > 16 {
        return Err(ValidationError::new("密码长度必须在8到16个字符之间"));
    }

    for ch in password.chars() {
        if ch.is_ascii_uppercase() {
            has_upper = true;
        } else if ch.is_ascii_lowercase() {
            has_lower = true;
        } else if ch.is_ascii_digit() {
            has_digit = true;
        }
    }

    if has_upper && has_lower && has_digit {
        Ok(())
    } else if !has_upper {
        return Err(ValidationError::new("密码至少包含一个大写字母"));
    } else if !has_lower {
        return Err(ValidationError::new("密码至少包含一个小写字母"));
    } else if !has_digit {
        return Err(ValidationError::new("密码至少包含一个数字"));
    } else {
        Ok(())
    }
}
