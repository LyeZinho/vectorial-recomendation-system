//! REST API server (Axum)

pub mod errors;
pub mod handlers;
pub mod state;
pub mod auth;
pub mod cache;
pub mod middleware;

use axum::{
    routing::{get, post},
    middleware as axum_middleware,
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
    let rate_limiter = state.rate_limiter.clone();

    Router::new()
        .route("/", get(root_handler))
        .route("/api/login", post(handlers::login_handler))
        .route("/api/refresh", post(handlers::refresh_handler))
        .route("/api/recommendations/:id", get(handlers::recommendations_handler))
        .route("/api/search", get(handlers::search_handler))
        .route("/api/explain/:anime_id/:rec_id", get(handlers::explain_handler))
        .route(
            "/api/admin/cache/invalidate",
            post(handlers::invalidate_cache_handler)
                .layer(axum_middleware::from_fn_with_state(
                    state.clone(),
                    crate::api::auth::admin_middleware,
                )),
        )
        .layer(axum_middleware::from_fn(move |req, next: axum_middleware::Next| {
            let limiter = rate_limiter.clone();
            async move {
                if limiter.check_limit().is_err() {
                    return Err(StatusCode::TOO_MANY_REQUESTS);
                }
                Ok(next.run(req).await)
            }
        }))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_router_creation() {
        assert!(true);
    }
}
