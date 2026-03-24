//! GET /api/recommendations/:id - Get recommendations for an anime

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::api::state::SharedState;
use crate::api::errors::ApiError;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct RecommendationResponse {
    pub anime_id: u64,
    pub anime_name: Option<String>,
    pub recommendations: Vec<RecommendationItem>,
    pub total: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RecommendationItem {
    pub anime_id: u64,
    pub name: String,
    pub genre: Option<String>,
    pub rating: Option<f32>,
    pub score: f32,
    pub weight: f32,
    pub layer: String,
}

pub async fn recommendations_handler(
    Path(anime_id): Path<u64>,
    State(state): State<SharedState>,
) -> Result<(StatusCode, Json<RecommendationResponse>), ApiError> {
    let cache_key = format!("recommendations:anime:{}", anime_id);

    if let Some(cache) = &state.cache {
        if let Ok(Some(cached_response)) = cache.get(&cache_key) {
            if let Ok(parsed) = serde_json::from_str::<RecommendationResponse>(&cached_response) {
                return Ok((StatusCode::OK, Json(parsed)));
            }
        }
    }

    let source_query = "MATCH (a:Anime {id: $id}) RETURN a.name as name LIMIT 1";
    let mut result = state
        .graph
        .execute(neo4rs::query(source_query).param("id", anime_id as i64))
        .await
        .map_err(|_| ApiError::ServiceUnavailable)?;

    let anime_name = if let Ok(Some(row)) = result.next().await {
        row.get::<Option<String>>("name").ok().flatten()
    } else {
        return Err(ApiError::NotFound(format!(
            "Anime with ID {} not found",
            anime_id
        )));
    };

    let rec_query = r#"
        MATCH (a:Anime {id: $id})
        WITH a
        MATCH (a)-[r]->(b:Anime)
        WHERE r.weight >= 0.7
        RETURN 
            b.id as anime_id,
            b.name as name,
            b.genre as genre,
            b.rating as rating,
            r.weight as weight,
            type(r) as rel_type
        ORDER BY r.weight DESC
        LIMIT 10
    "#;

    let mut recommendations = Vec::new();
    let mut result = state
        .graph
        .execute(neo4rs::query(rec_query).param("id", anime_id as i64))
        .await
        .map_err(|_| ApiError::ServiceUnavailable)?;

    while let Ok(Some(row)) = result.next().await {
        let rec_id: i64 = row.get("anime_id").unwrap_or(0);
        let name: String = row.get("name").unwrap_or_default();
        let genre: Option<String> = row.get("genre").ok().flatten();
        let rating: Option<f32> = row.get("rating").ok().flatten();
        let weight: f32 = row.get("weight").unwrap_or(0.0);
        let rel_type: String = row.get("rel_type").unwrap_or_else(|_| "RELATED_TO".to_string());

        let layer = match rel_type.as_str() {
            "RELATED_TO" => "technical_dna",
            "SIMILAR" => "niche_clustering",
            "INFLUENCED_BY" => "influence_chain",
            _ => "underground_discovery",
        };

        let score = (weight * 0.6) + ((rating.unwrap_or(0.0) / 5.0) * 0.4);

        recommendations.push(RecommendationItem {
            anime_id: rec_id as u64,
            name,
            genre,
            rating,
            score: score.min(1.0).max(0.0),
            weight,
            layer: layer.to_string(),
        });
    }

    let response = RecommendationResponse {
        anime_id,
        anime_name,
        total: recommendations.len(),
        recommendations,
    };

    if let Some(cache) = &state.cache {
        if let Ok(json) = serde_json::to_string(&response) {
            let _ = cache.set_ex(&cache_key, &json, 3600);
        }
    }

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_serialization() {
        let resp = RecommendationResponse {
            anime_id: 1,
            anime_name: Some("Test Anime".to_string()),
            recommendations: vec![],
            total: 0,
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"anime_id\":1"));
    }
}
