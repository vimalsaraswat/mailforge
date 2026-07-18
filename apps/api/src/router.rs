use axum::{Router, routing::get};

use crate::controllers;

pub fn router() -> Router {
    Router::new().route("/health", get(controllers::health::health))
}
