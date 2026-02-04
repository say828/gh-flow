// User registration functionality
use super::base::AuthConfig;

pub struct SignupData {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn register_user(data: &SignupData, config: &AuthConfig) -> Result<String, String> {
    // Validate input
    if data.username.len() < 3 {
        return Err("Username too short".to_string());
    }
    if !data.email.contains('@') {
        return Err("Invalid email".to_string());
    }
    if data.password.len() < 8 {
        return Err("Password too short".to_string());
    }

    Ok(format!("User {} registered", data.username))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signup_success() {
        let config = AuthConfig::new("secret");
        let data = SignupData {
            username: "newuser".to_string(),
            email: "user@example.com".to_string(),
            password: "password123".to_string(),
        };
        assert!(register_user(&data, &config).is_ok());
    }

    #[test]
    fn test_signup_invalid_email() {
        let config = AuthConfig::new("secret");
        let data = SignupData {
            username: "newuser".to_string(),
            email: "invalid".to_string(),
            password: "password123".to_string(),
        };
        assert!(register_user(&data, &config).is_err());
    }
}
