use crate::{controllers, state::AppState};
use axum::{Router, routing::get};

pub fn template_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(controllers::email_template::list).post(controllers::email_template::create),
        )
        .route(
            "/{id}",
            get(controllers::email_template::get)
                .put(controllers::email_template::update)
                .delete(controllers::email_template::delete),
        )
}
