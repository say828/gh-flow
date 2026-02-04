// Authentication Base Infrastructure
// This provides the foundation for all auth features

pub struct AuthConfig {
    pub secret_key: String,
    pub token_expiry: u64,
}

impl AuthConfig {
    pub fn new(secret: &str) -> Self {
        Self {
            secret_key: secret.to_string(),
            token_expiry: 3600, // 1 hour
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_config_creation() {
        let config = AuthConfig::new("test_secret");
        assert_eq!(config.secret_key, "test_secret");
        assert_eq!(config.token_expiry, 3600);
    }
}
