use axum::{
    Router,
    routing::{get, post},
};

use crate::{controllers, http, state::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(controllers::health::health))
        .nest("/auth", auth_router())
        .nest("/templates", template_router())
        .layer(http::cors::layer(&state.config))
        .with_state(state)
}

fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/me", get(controllers::auth::me))
        .route("/logout", post(controllers::auth::logout))
        .route("/google", get(controllers::auth::google_login))
        .route("/google/callback", get(controllers::auth::google_callback))
}

fn template_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(controllers::email_template::list).post(controllers::email_template::create),
        )
        .route(
            "/:id",
            get(controllers::email_template::get)
                .put(controllers::email_template::update)
                .delete(controllers::email_template::delete),
        )
}
