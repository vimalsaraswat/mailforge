use axum::{Router, routing::get};

use crate::{controllers, state::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(controllers::health::health))
        .with_state(state)
}
