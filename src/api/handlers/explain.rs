//! GET /api/explain/:anime_id/:rec_id - Explain why a recommendation was made

use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Serialize;
use crate::api::state::SharedState;
use crate::api::errors::ApiError;

#[derive(Serialize)]
pub struct ExplainResponse {
    pub source_id: u64,
    pub source_name: Option<String>,
    pub recommendation_id: u64,
    pub recommendation_name: Option<String>,
    pub relationship: String,
    pub layer: String,
    pub weight: f32,
    pub explanation: String,
}

pub async fn explain_handler(
    Path((anime_id, rec_id)): Path<(u64, u64)>,
    State(state): State<SharedState>,
) -> Result<(StatusCode, Json<ExplainResponse>), ApiError> {
    let query = r#"
        MATCH (a:Anime {id: $anime_id})-[r]->(b:Anime {id: $rec_id})
        RETURN 
            a.name as source_name,
            b.name as rec_name,
            type(r) as rel_type,
            r.weight as weight
        LIMIT 1
    "#;

    let mut result = state
        .graph
        .execute(
            neo4rs::query(query)
                .param("anime_id", anime_id as i64)
                .param("rec_id", rec_id as i64),
        )
        .await
        .map_err(|_| ApiError::ServiceUnavailable)?;

    if let Ok(Some(row)) = result.next().await {
        let source_name: Option<String> = row.get("source_name").ok().flatten();
        let rec_name: Option<String> = row.get("rec_name").ok().flatten();
        let rel_type: String = row.get("rel_type").unwrap_or_else(|_| "RELATED_TO".to_string());
        let weight: f32 = row.get("weight").unwrap_or(0.0);

        let (layer, explanation) = match rel_type.as_str() {
            "RELATED_TO" => (
                "technical_dna".to_string(),
                format!(
                    "These anime share similar themes and narrative structure (relationship weight: {:.2})",
                    weight
                ),
            ),
            "SIMILAR" => (
                "niche_clustering".to_string(),
                format!(
                    "Genre bridge discovery - combines similar elements with complementary themes (weight: {:.2})",
                    weight
                ),
            ),
            "INFLUENCED_BY" => (
                "influence_chain".to_string(),
                format!(
                    "This anime was influenced by industry predecessors (weight: {:.2})",
                    weight
                ),
            ),
            _ => (
                "underground_discovery".to_string(),
                "Hidden gem with quality comparable to your selection".to_string(),
            ),
        };

        Ok((
            StatusCode::OK,
            Json(ExplainResponse {
                source_id: anime_id,
                source_name,
                recommendation_id: rec_id,
                recommendation_name: rec_name,
                relationship: rel_type,
                layer,
                weight,
                explanation,
            }),
        ))
    } else {
        Err(ApiError::NotFound(format!(
            "Relationship between anime {} and {} not found",
            anime_id, rec_id
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explain_response_serialization() {
        let resp = ExplainResponse {
            source_id: 1,
            source_name: Some("Anime A".to_string()),
            recommendation_id: 2,
            recommendation_name: Some("Anime B".to_string()),
            relationship: "RELATED_TO".to_string(),
            layer: "technical_dna".to_string(),
            weight: 0.85,
            explanation: "Test explanation".to_string(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("technical_dna"));
    }
}
