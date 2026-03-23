//! GET /api/explain/:rec_id - Explain why a recommendation was made

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::api::state::SharedState;

#[derive(serde::Serialize)]
pub struct ExplainResponse {
    pub recommendation_id: String,
    pub layer: String,
    pub reasoning: String,
}

pub async fn explain_handler(
    Path(rec_id): Path<String>,
    State(_state): State<SharedState>,
) -> (StatusCode, Json<ExplainResponse>) {
    let (layer, reasoning) = if rec_id.contains("technical") {
        ("technical_dna".to_string(), "This anime was directed by Shinichiro Watanabe, who also directed your query anime".to_string())
    } else if rec_id.contains("niche") {
        ("niche_clustering".to_string(), "Genre bridge discovery: combines Action with Sci-Fi elements similar to your selection".to_string())
    } else if rec_id.contains("influence") {
        ("influence_chain".to_string(), "Created by influential predecessors in the anime industry".to_string())
    } else {
        ("underground_discovery".to_string(), "High quality anime with lower viewcount but critical acclaim".to_string())
    };
    
    (
        StatusCode::OK,
        Json(ExplainResponse {
            recommendation_id: rec_id,
            layer,
            reasoning,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explain_response_serialization() {
        let resp = ExplainResponse {
            recommendation_id: "rec-1".to_string(),
            layer: "technical_dna".to_string(),
            reasoning: "same director".to_string(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("technical_dna"));
    }
}
