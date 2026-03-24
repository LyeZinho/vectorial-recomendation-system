use crate::api::auth::types::TokenClaims;
use crate::api::errors::ApiError;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

pub struct JwtManager {
    secret: String,
}

impl JwtManager {
    pub fn new(secret: String) -> Self {
        if secret.len() < 32 {
            panic!("JWT secret must be at least 32 characters");
        }
        Self { secret }
    }

    pub fn create_access_token(
        &self,
        user_id: String,
        user_type: String,
    ) -> Result<String, ApiError> {
        let claims = TokenClaims::new(user_id, user_type, 1);
        let encoding_key = EncodingKey::from_secret(self.secret.as_bytes());

        encode(&Header::default(), &claims, &encoding_key)
            .map_err(|_| ApiError::InternalError("Failed to create access token".to_string()))
    }

    pub fn create_refresh_token(&self, user_id: String) -> Result<String, ApiError> {
        let claims = TokenClaims::new(user_id, "user".to_string(), 168);
        let encoding_key = EncodingKey::from_secret(self.secret.as_bytes());

        encode(&Header::default(), &claims, &encoding_key)
            .map_err(|_| ApiError::InternalError("Failed to create refresh token".to_string()))
    }

    pub fn verify_token(&self, token: &str) -> Result<TokenClaims, ApiError> {
        let decoding_key = DecodingKey::from_secret(self.secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        decode::<TokenClaims>(token, &decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|_| ApiError::BadRequest("Invalid or expired token".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_token() {
        let manager = JwtManager::new("x".repeat(64));
        let token = manager
            .create_access_token("user123".to_string(), "user".to_string())
            .unwrap();
        let claims = manager.verify_token(&token).unwrap();

        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.user_type, "user");
        assert!(!claims.is_expired());
    }

    #[test]
    fn test_admin_claim_detection() {
        let manager = JwtManager::new("x".repeat(64));
        let token = manager
            .create_access_token("admin1".to_string(), "admin".to_string())
            .unwrap();
        let claims = manager.verify_token(&token).unwrap();

        assert!(claims.is_admin());
    }
}
