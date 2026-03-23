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
    (
        StatusCode::OK,
        Json(ExplainResponse {
            recommendation_id: rec_id,
            layer: "technical_dna".to_string(),
            reasoning: "placeholder".to_string(),
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
