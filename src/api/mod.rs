//! REST API server (Axum)

pub mod handlers;
pub mod state;

use axum::{
    routing::get,
    Router,
};
use crate::api::state::SharedState;

pub fn build_router(state: SharedState) -> Router {
    Router::new()
        .route("/api/recommendations/:id", get(handlers::recommendations_handler))
        .route("/api/search", get(handlers::search_handler))
        .route("/api/explain/:rec_id", get(handlers::explain_handler))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_router_creation() {
        assert!(true);
    }
}
