mod auth;
mod templates;

use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;

use crate::{controllers, http, state::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(controllers::health::health))
        .nest("/auth", auth::auth_router())
        .nest("/templates", templates::template_router())
        .layer(http::cors::layer(&state.config))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
