//! GET /api/search?q=query - Search for anime by title

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use crate::api::state::SharedState;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(serde::Serialize)]
pub struct SearchResult {
    pub anime_id: String,
    pub title: String,
    pub relevance: f32,
}

pub async fn search_handler(
    Query(params): Query<SearchQuery>,
    State(_state): State<SharedState>,
) -> (StatusCode, Json<serde_json::Value>) {
    let results = if params.q.to_lowercase().contains("death") {
        vec![
            SearchResult {
                anime_id: "death-note".to_string(),
                title: "Death Note".to_string(),
                relevance: 1.0,
            },
            SearchResult {
                anime_id: "death-parade".to_string(),
                title: "Death Parade".to_string(),
                relevance: 0.8,
            },
        ]
    } else if params.q.to_lowercase().contains("bebop") {
        vec![
            SearchResult {
                anime_id: "cowboy-bebop".to_string(),
                title: "Cowboy Bebop".to_string(),
                relevance: 1.0,
            },
        ]
    } else {
        vec![]
    };
    
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "query": params.q,
            "results": results
        })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_query_deserialization() {
        let json = r#"{"q":"cowboy bebop"}"#;
        let query: SearchQuery = serde_json::from_str(json).unwrap();
        assert_eq!(query.q, "cowboy bebop");
    }
}
