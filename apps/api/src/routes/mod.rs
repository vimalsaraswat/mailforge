use axum::{Router, routing::get};

use crate::{controllers, http, state::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(controllers::health::health))
        .nest("/auth", auth_router())
        .layer(http::cors::layer(&state.config))
        .with_state(state)
}

fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/me", get(controllers::auth::me))
        .route("/google", get(controllers::auth::google_login))
        .route("/google/callback", get(controllers::auth::google_callback))
}
