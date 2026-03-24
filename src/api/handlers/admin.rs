use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::api::state::SharedState;
use crate::api::auth::AuthUser;
use crate::api::errors::ApiError;

#[derive(Deserialize)]
pub struct InvalidateCacheRequest {
    pub pattern: String,
}

#[derive(Serialize)]
pub struct InvalidateCacheResponse {
    pub message: String,
}

pub async fn invalidate_cache_handler(
    State(state): State<SharedState>,
    AuthUser(user): AuthUser,
    Json(req): Json<InvalidateCacheRequest>,
) -> Result<(StatusCode, Json<InvalidateCacheResponse>), ApiError> {
    if !user.is_admin() {
        return Err(ApiError::BadRequest("Admin access required".to_string()));
    }

    if let Some(cache) = &state.cache {
        cache
            .invalidate_pattern(&req.pattern)
            .map_err(|_| ApiError::InternalError("Failed to invalidate cache".to_string()))?;
    }

    Ok((
        StatusCode::OK,
        Json(InvalidateCacheResponse {
            message: format!("Cache invalidated for pattern: {}", req.pattern),
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalidate_response_serialization() {
        let resp = InvalidateCacheResponse {
            message: "Cache cleared".to_string(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("Cache cleared"));
    }
}
