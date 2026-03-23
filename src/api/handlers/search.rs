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
pub struct SearchResponse {
    pub results: Vec<serde_json::Value>,
}

pub async fn search_handler(
    Query(params): Query<SearchQuery>,
    State(_state): State<SharedState>,
) -> (StatusCode, Json<SearchResponse>) {
    let _ = params.q;
    (StatusCode::OK, Json(SearchResponse { results: vec![] }))
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
