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

pub async fn google_login(
    State(state): State<AppState>,
    Query(query): Query<std::collections::HashMap<String, String>>,
) -> Response {
    let include_gmail = query.get("connect").map(|v| v == "true").unwrap_or(false);

    let (authorization_url, csrf_state, pkce_verifier) =
        AuthService::new(&state.db, state.config.clone()).start_google_login(include_gmail);

    let cookie_manager = cookies::CookieManager::new(state.config.clone());
    let oauth_cookie =
        cookie_manager.oauth_flow_cookie(csrf_state.secret(), pkce_verifier.secret(), include_gmail);

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
    let auth_service = AuthService::new(&state.db, state.config.clone());

    let failure = |status: StatusCode, message: String| {
        let mut response = (status, message).into_response();
        response.headers_mut().append(
            header::SET_COOKIE,
            cookies::cookie_header(&cookie_manager.expired_oauth_flow_cookie()),
        );
        response
    };

    if query.error.is_some() {
        tracing::warn!("Google OAuth provider returned an authorization error");
        return failure(
            StatusCode::BAD_REQUEST,
            "Google authorization was denied".to_string(),
        );
    }

    let code = match query.code {
        Some(code) => code,
        None => {
            return failure(
                StatusCode::BAD_REQUEST,
                "Missing Google authorization code".to_string(),
            );
        }
    };
    let returned_state = match query.state {
        Some(state) => state,
        None => return failure(StatusCode::BAD_REQUEST, "Missing OAuth state".to_string()),
    };
    let (expected_state, pkce_verifier, should_connect) = match cookie_manager.oauth_flow(&headers) {
        Some(flow) => flow,
        None => return failure(StatusCode::BAD_REQUEST, "Missing OAuth session".to_string()),
    };
    if returned_state != expected_state {
        return failure(StatusCode::BAD_REQUEST, "Invalid OAuth state".to_string());
    }

    let login = match auth_service
        .complete_google_login(code, pkce_verifier, should_connect)
        .await
    {
        Ok(login) => login,
        Err(error) => {
            let mut response = errors::auth(error);
            response.headers_mut().append(
                header::SET_COOKIE,
                cookies::cookie_header(&cookie_manager.expired_oauth_flow_cookie()),
            );
            return response;
        }
    };

    let mut response = Redirect::to(&state.config.frontend_url).into_response();
    let cookies = vec![
        cookies::cookie_header(&cookie_manager.session_cookie(&login.session_id)),
        cookies::cookie_header(&cookie_manager.expired_oauth_flow_cookie()),
    ];

    response
        .headers_mut()
        .extend(cookies.into_iter().map(|c| (header::SET_COOKIE, c)));

    response
}

pub async fn me(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let cookie_manager = cookies::CookieManager::new(state.config.clone());
    let Some(session_id) = cookie_manager.session_id(&headers) else {
        return StatusCode::UNAUTHORIZED.into_response();
    };

    let auth_service = AuthService::new(&state.db, state.config.clone());
    let user = match auth_service.current_user(session_id).await {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(error) => return errors::auth(error),
    };
    let gmail_status = auth_service.get_gmail_connection_status(user.id).await.unwrap_or(None);

    let mut response = MeResponse::from(user);
    response.gmail_connected = gmail_status.map(|s| s.0);
    response.gmail_connected_at = gmail_status.map(|s| s.1);

    Json(response).into_response()
}

pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let cookie_manager = cookies::CookieManager::new(state.config.clone());

    // Remove the session from storage if it exists.
    if let Some(session_id) = cookie_manager.session_id(&headers) {
        let auth_service = AuthService::new(&state.db, state.config.clone());

        if let Err(err) = auth_service.logout(session_id).await {
            return errors::auth(err);
        }
    }

    let mut response = StatusCode::NO_CONTENT.into_response();

    response.headers_mut().insert(
        header::SET_COOKIE,
        cookies::cookie_header(&cookie_manager.expired_session_cookie()),
    );

    response
}
