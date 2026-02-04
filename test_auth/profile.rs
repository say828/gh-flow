// User profile management
pub struct UserProfile {
    pub user_id: String,
    pub display_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

impl UserProfile {
    pub fn new(user_id: &str, email: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            display_name: user_id.to_string(),
            email: email.to_string(),
            avatar_url: None,
        }
    }

    pub fn update_display_name(&mut self, new_name: String) {
        self.display_name = new_name;
    }

    pub fn set_avatar(&mut self, url: String) {
        self.avatar_url = Some(url);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_creation() {
        let profile = UserProfile::new("user123", "user@example.com");
        assert_eq!(profile.user_id, "user123");
        assert_eq!(profile.display_name, "user123");
    }

    #[test]
    fn test_profile_update() {
        let mut profile = UserProfile::new("user123", "user@example.com");
        profile.update_display_name("New Name".to_string());
        assert_eq!(profile.display_name, "New Name");
    }
}
