//! Web module: static file serving for dashboard

use axum::{
    body::Body,
    http::{StatusCode, header},
    response::IntoResponse,
    Router,
    routing::get,
};
use std::path::PathBuf;
use tower_http::services::ServeDir;

pub fn build_static_router() -> Router {
    Router::new()
        .nest_service("/", ServeDir::new("src/web/static"))
        .fallback(handle_404)
}

async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "application/json")],
        r#"{"error":"Not found"}"#,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_module_exists() {
        assert!(true);
    }
}
