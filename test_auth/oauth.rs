// OAuth integration for third-party authentication
use super::base::AuthConfig;

pub enum OAuthProvider {
    Google,
    GitHub,
    Facebook,
}

pub struct OAuthToken {
    pub provider: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

pub fn oauth_authenticate(
    provider: OAuthProvider,
    code: &str,
    config: &AuthConfig,
) -> Result<OAuthToken, String> {
    if code.is_empty() {
        return Err("Invalid OAuth code".to_string());
    }

    let provider_name = match provider {
        OAuthProvider::Google => "google",
        OAuthProvider::GitHub => "github",
        OAuthProvider::Facebook => "facebook",
    };

    Ok(OAuthToken {
        provider: provider_name.to_string(),
        access_token: format!("{}_{}", provider_name, code),
        refresh_token: Some("refresh_token".to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_google() {
        let config = AuthConfig::new("secret");
        let result = oauth_authenticate(OAuthProvider::Google, "auth_code", &config);
        assert!(result.is_ok());
    }
}
