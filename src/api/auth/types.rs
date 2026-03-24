use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub user_type: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

impl TokenClaims {
    pub fn new(sub: String, user_type: String, hours: i64) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::hours(hours)).timestamp();
        let iat = now.timestamp();

        Self {
            sub,
            exp,
            iat,
            user_type,
        }
    }

    pub fn is_admin(&self) -> bool {
        self.user_type == "admin"
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}
