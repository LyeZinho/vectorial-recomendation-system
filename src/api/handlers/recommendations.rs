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
    pub recommendations: Vec<RecommendationItem>,
}

#[derive(serde::Serialize)]
pub struct RecommendationItem {
    pub anime_id: String,
    pub title: String,
    pub score: f32,
    pub layer: String,
    pub explanation: String,
}

pub async fn recommendations_handler(
    Path(anime_id): Path<String>,
    State(_state): State<SharedState>,
) -> (StatusCode, Json<RecommendationResponse>) {
    let recommendations = vec![
        RecommendationItem {
            anime_id: "rec1".to_string(),
            title: "Similar Anime 1".to_string(),
            score: 0.95,
            layer: "technical_dna".to_string(),
            explanation: "Same director".to_string(),
        },
        RecommendationItem {
            anime_id: "rec2".to_string(),
            title: "Similar Anime 2".to_string(),
            score: 0.87,
            layer: "niche_clustering".to_string(),
            explanation: "Genre bridge discovery".to_string(),
        },
    ];
    
    (
        StatusCode::OK,
        Json(RecommendationResponse {
            anime_id,
            recommendations,
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
