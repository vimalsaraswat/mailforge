use axum::http::HeaderValue;
use tower_http::cors::{Any, CorsLayer};

use crate::config::Config;

pub fn layer(config: &Config) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(
            config
                .frontend_url
                .parse::<HeaderValue>()
                .expect("FRONTEND_URL must be a valid origin"),
        )
        .allow_methods(Any)
        .allow_credentials(true)
}
