use crate::{controllers, state::AppState};
use axum::{
    Router,
    routing::{get, post},
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/me", get(controllers::auth::me))
        .route("/logout", post(controllers::auth::logout))
        .route("/google", get(controllers::auth::google_login))
        .route("/google/callback", get(controllers::auth::google_callback))
}
