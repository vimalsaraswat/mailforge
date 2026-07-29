use axum::{
    Json,
    extract::{Query, State},
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

pub async fn google_callback(
    State(state): State<AppState>,
    Query(query): Query<GoogleCallbackQuery>,
    headers: HeaderMap,
) -> Response {
    let cookie_manager = cookies::CookieManager::new(state.config.clone());

    let failure = |response: Response| {
        let mut response = response;
        response.headers_mut().append(
            header::SET_COOKIE,
            cookies::cookie_header(&cookie_manager.expired_oauth_flow_cookie()),
        );
        response
    };

    if let Some(error) = query.error {
        return failure((StatusCode::BAD_REQUEST, error).into_response());
    }

    let Some(code) = query.code else {
        return failure(
            (StatusCode::BAD_REQUEST, "Missing Google authorization code").into_response(),
        );
    };
    let Some(returned_state) = query.state else {
        return failure((StatusCode::BAD_REQUEST, "Missing OAuth state").into_response());
    };
    let Some((expected_state, pkce_verifier)) = cookie_manager.oauth_flow(&headers) else {
        return failure((StatusCode::BAD_REQUEST, "Missing OAuth session").into_response());
    };
    if returned_state != expected_state {
        return failure((StatusCode::BAD_REQUEST, "Invalid OAuth state").into_response());
    }

    let login = match AuthService::new(&state.db, state.config.clone())
        .complete_google_login(code, pkce_verifier)
        .await
    {
        Ok(login) => login,
        Err(error) => return failure(errors::auth(error)),
    };

    let mut response = Redirect::to(&state.config.frontend_url).into_response();
    response.headers_mut().append(
        header::SET_COOKIE,
        cookies::cookie_header(&cookie_manager.session_cookie(&login.session_id)),
    );
    response.headers_mut().append(
        header::SET_COOKIE,
        cookies::cookie_header(&cookie_manager.expired_oauth_flow_cookie()),
    );
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
