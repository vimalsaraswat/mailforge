use axum::http::{HeaderValue, Method, header};
use tower_http::cors::CorsLayer;

use crate::config::Config;

pub fn layer(config: &Config) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(
            config
                .frontend_url
                .parse::<HeaderValue>()
                .expect("FRONTEND_URL must be a valid origin"),
        )
        .allow_headers([
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_credentials(true)
}
