use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};

use crate::{
    controllers::errors, dto::auth::MeResponse, http::cookies, services::auth::AuthService,
    state::AppState,
};

pub async fn me(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let cookie_manager = cookies::CookieManager::new(state.config.clone());
    let Some(session_id) = cookie_manager.session_id(&headers) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let user = match AuthService::new(&state.db, state.config.clone())
        .current_user(session_id)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(error) => return errors::auth(error),
    };
    Json(MeResponse::from(user)).into_response()
}
