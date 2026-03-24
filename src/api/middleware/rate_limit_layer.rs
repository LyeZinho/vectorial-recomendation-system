use axum::{
    http::StatusCode,
    middleware::Next,
    response::Response,
    body::Body,
    http::Request,
};
use crate::api::middleware::RateLimitManager;

pub async fn rate_limit_middleware(
    rate_limiter: RateLimitManager,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if rate_limiter.check_limit().is_err() {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}
