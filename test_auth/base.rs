// Authentication Base Infrastructure
// This provides the foundation for all auth features

pub struct AuthConfig {
    pub secret_key: String,
    pub token_expiry: u64,
    pub max_attempts: u32,  // NEW: Rate limiting
    pub lockout_duration: u64,  // NEW: Lockout time in seconds
}

impl AuthConfig {
    pub fn new(secret: &str) -> Self {
        Self {
            secret_key: secret.to_string(),
            token_expiry: 3600, // 1 hour
            max_attempts: 5,    // NEW: Max 5 failed attempts
            lockout_duration: 900, // NEW: 15 minutes lockout
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
// Testing dry-run
