use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::api::state::SharedState;
use crate::api::auth::{TokenResponse, RefreshTokenRequest};
use crate::api::errors::ApiError;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub user_id: String,
    pub password: String, // TODO: validate against real user DB in Phase 5+
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub async fn login_handler(
    State(state): State<SharedState>,
    Json(req): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), ApiError> {
    if req.user_id.is_empty() || req.password.is_empty() {
        return Err(ApiError::BadRequest(
            "user_id and password are required".to_string(),
        ));
    }

    // TODO: Validate password against user database
    // For now, accept any non-empty credentials
    let access_token = state.jwt_manager.create_access_token(
        req.user_id.clone(),
        "user".to_string(),
    )?;

    let refresh_token = state.jwt_manager.create_refresh_token(req.user_id)?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            access_token,
            refresh_token,
            expires_in: 3600,
        }),
    ))
}

pub async fn refresh_handler(
    State(state): State<SharedState>,
    Json(req): Json<RefreshTokenRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), ApiError> {
    let claims = state.jwt_manager.verify_token(&req.refresh_token)?;

    let access_token = state.jwt_manager.create_access_token(
        claims.sub.clone(),
        claims.user_type,
    )?;

    let refresh_token = state.jwt_manager.create_refresh_token(claims.sub)?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            access_token,
            refresh_token,
            expires_in: 3600,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_response_serialization() {
        let resp = LoginResponse {
            access_token: "token123".to_string(),
            refresh_token: "refresh123".to_string(),
            expires_in: 3600,
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("access_token"));
    }
}
