//! GET /api/search?q=query - Search for anime by title

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::api::state::SharedState;
use crate::api::errors::ApiError;

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize, Clone)]
pub struct SearchResult {
    pub anime_id: u64,
    pub name: String,
    pub genre: Option<String>,
    pub relevance: f32,
}

#[derive(Serialize, Clone)]
pub struct SearchSuggestion {
    pub anime_id: u64,
    pub name: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub suggestions: Vec<SearchSuggestion>,
}

pub async fn search_handler(
    Query(params): Query<SearchQuery>,
    State(state): State<SharedState>,
) -> Result<(StatusCode, Json<SearchResponse>), ApiError> {
    if params.q.is_empty() {
        return Err(ApiError::BadRequest(
            "Search query cannot be empty".to_string(),
        ));
    }

    let query_upper = params.q.to_uppercase();

    let search_query = r#"
        MATCH (a:Anime)
        WHERE a.name =~ '(?i).*' + $query + '.*'
        RETURN 
            a.id as anime_id,
            a.name as name,
            a.genre as genre
        ORDER BY 
            CASE WHEN a.name =~ '(?i)^' + $query THEN 0 ELSE 1 END,
            a.name
        LIMIT 20
    "#;

    let mut results = Vec::new();
    let mut result = state
        .graph
        .execute(
            neo4rs::query(search_query)
                .param("query", params.q.clone()),
        )
        .await
        .map_err(|_| ApiError::ServiceUnavailable)?;

    while let Ok(Some(row)) = result.next().await {
        let anime_id: i64 = row.get("anime_id").unwrap_or(0);
        let name: String = row.get("name").unwrap_or_default();
        let genre: Option<String> = row.get("genre").ok().flatten();

        let relevance = if name.to_lowercase().starts_with(&params.q.to_lowercase()) {
            1.0
        } else if name.to_lowercase().contains(&params.q.to_lowercase()) {
            0.8
        } else {
            0.6
        };

        results.push(SearchResult {
            anime_id: anime_id as u64,
            name,
            genre,
            relevance,
        });
    }

    let suggest_query = r#"
        MATCH (a:Anime)
        WHERE a.name =~ '(?i)^' + $query
        RETURN a.id as anime_id, a.name as name
        LIMIT 5
    "#;

    let mut suggestions = Vec::new();
    let mut result = state
        .graph
        .execute(
            neo4rs::query(suggest_query)
                .param("query", params.q.clone()),
        )
        .await
        .map_err(|_| ApiError::ServiceUnavailable)?;

    while let Ok(Some(row)) = result.next().await {
        let anime_id: i64 = row.get("anime_id").unwrap_or(0);
        let name: String = row.get("name").unwrap_or_default();
        suggestions.push(SearchSuggestion {
            anime_id: anime_id as u64,
            name,
        });
    }

    Ok((
        StatusCode::OK,
        Json(SearchResponse {
            query: params.q,
            results,
            suggestions,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_response_serialization() {
        let resp = SearchResponse {
            query: "test".to_string(),
            results: vec![],
            suggestions: vec![],
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("\"query\":\"test\""));
    }
}
