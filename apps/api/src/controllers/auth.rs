use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::{
    controllers::errors, dto::auth::MeResponse, http::cookies, services::auth::AuthService,
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

pub async fn google_login(State(state): State<AppState>) -> Response {
    let (authorization_url, csrf_state, pkce_verifier) =
        AuthService::new(&state.db, state.config.clone()).start_google_login();

    let cookie_manager = cookies::CookieManager::new(state.config.clone());
    let oauth_cookie =
        cookie_manager.oauth_flow_cookie(csrf_state.secret(), pkce_verifier.secret());

    let mut response = Redirect::to(authorization_url.as_str()).into_response();
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookies::cookie_header(&oauth_cookie));

    response
}

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
