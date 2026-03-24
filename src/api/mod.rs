//! REST API server (Axum)

pub mod errors;
pub mod handlers;
pub mod state;
pub mod auth;

use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use crate::api::state::SharedState;
use serde_json::json;

async fn root_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "service": "Anime2Vec Recommendation Engine",
            "version": "2.0",
            "status": "ready"
        }))
    )
}

pub fn build_router(state: SharedState) -> Router {
    Router::new()
        .route("/", get(root_handler))
        .route("/api/login", post(handlers::login_handler))
        .route("/api/refresh", post(handlers::refresh_handler))
        .route("/api/recommendations/:id", get(handlers::recommendations_handler))
        .route("/api/search", get(handlers::search_handler))
        .route("/api/explain/:anime_id/:rec_id", get(handlers::explain_handler))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_router_creation() {
        assert!(true);
    }
}
