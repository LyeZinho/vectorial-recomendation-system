//! GET /api/recommendations/:id - Get recommendations for an anime

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::api::state::SharedState;

#[derive(serde::Serialize)]
pub struct RecommendationResponse {
    pub anime_id: String,
    pub recommendations: Vec<serde_json::Value>,
}

pub async fn recommendations_handler(
    Path(anime_id): Path<String>,
    State(_state): State<SharedState>,
) -> (StatusCode, Json<RecommendationResponse>) {
    (
        StatusCode::OK,
        Json(RecommendationResponse {
            anime_id,
            recommendations: vec![],
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_serialization() {
        let resp = RecommendationResponse {
            anime_id: "test-id".to_string(),
            recommendations: vec![],
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("test-id"));
    }
}
