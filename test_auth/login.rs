// Login functionality
use super::base::AuthConfig;

pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

pub fn authenticate(creds: &LoginCredentials, config: &AuthConfig) -> Result<String, String> {
    // Simplified authentication logic
    if creds.username.is_empty() || creds.password.is_empty() {
        return Err("Invalid credentials".to_string());
    }

    // Generate a simple token (in real app, use JWT)
    let token = format!("{}_{}", creds.username, config.secret_key);
    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_success() {
        let config = AuthConfig::new("secret");
        let creds = LoginCredentials {
            username: "user1".to_string(),
            password: "pass123".to_string(),
        };
        assert!(authenticate(&creds, &config).is_ok());
    }

    #[test]
    fn test_login_failure() {
        let config = AuthConfig::new("secret");
        let creds = LoginCredentials {
            username: "".to_string(),
            password: "pass123".to_string(),
        };
        assert!(authenticate(&creds, &config).is_err());
    }
}
