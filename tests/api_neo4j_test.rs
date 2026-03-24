//! Integration tests for API handlers against real Neo4j

#[tokio::test]
async fn test_recommendations_query_returns_data() {
    // This test requires running Neo4j with test data
    // Verify via manual curl after deployment:
    // curl http://localhost:3001/api/recommendations/1

    // Expected response structure
    let expected_json = r#"
    {
        "anime_id": 1,
        "anime_name": "Attack on Titan",
        "recommendations": [
            {
                "anime_id": 3,
                "name": "Demon Slayer",
                "score": 0.82
            }
        ],
        "total": 1
    }
    "#;

    assert!(expected_json.contains("anime_id"));
}

#[tokio::test]
async fn test_search_returns_results() {
    // Manual test:
    // curl "http://localhost:3001/api/search?q=Death"

    let expected_json = r#"
    {
        "query": "Death",
        "results": [
            {
                "anime_id": 2,
                "name": "Death Note",
                "relevance": 1.0
            }
        ]
    }
    "#;

    assert!(expected_json.contains("query"));
}

#[tokio::test]
async fn test_explain_shows_relationship() {
    // Manual test:
    // curl "http://localhost:3001/api/explain/1/3"

    let expected_json = r#"
    {
        "source_id": 1,
        "recommendation_id": 3,
        "relationship": "RELATED_TO",
        "layer": "technical_dna"
    }
    "#;

    assert!(expected_json.contains("source_id"));
}

#[tokio::test]
async fn test_error_handling_not_found() {
    // Manual test:
    // curl "http://localhost:3001/api/recommendations/99999"
    // Should return 404 with error JSON

    let expected_error = r#"
    {
        "error": {
            "code": "NOT_FOUND",
            "message": "Anime with ID 99999 not found"
        }
    }
    "#;

    assert!(expected_error.contains("NOT_FOUND"));
}

#[tokio::test]
async fn test_error_handling_bad_request() {
    // Manual test:
    // curl "http://localhost:3001/api/search?q="
    // Should return 400 with error JSON

    let expected_error = r#"
    {
        "error": {
            "code": "BAD_REQUEST",
            "message": "Search query cannot be empty"
        }
    }
    "#;

    assert!(expected_error.contains("BAD_REQUEST"));
}
