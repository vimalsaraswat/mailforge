use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::{
    controllers::errors, dto::auth::MeResponse, http::cookies, middleware::AuthenticatedUser,
    services::auth::AuthService, state::AppState,
};

#[derive(Debug, Default, Deserialize)]
pub struct GoogleLoginQuery {
    pub connect: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

pub async fn google_login(
    State(state): State<AppState>,
    Query(query): Query<GoogleLoginQuery>,
) -> Response {
    let include_gmail = query.connect.unwrap_or(false);

    let (authorization_url, csrf_state, pkce_verifier) =
        AuthService::new(&state.db, state.config.clone()).start_google_login(include_gmail);

    let cookie_manager = cookies::CookieManager::new(state.config.clone());
    let oauth_cookie = cookie_manager.oauth_flow_cookie(
        csrf_state.secret(),
        pkce_verifier.secret(),
        include_gmail,
    );

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

    let with_cleared_oauth_cookie = |mut response: Response| {
        response.headers_mut().append(
            header::SET_COOKIE,
            cookies::cookie_header(&cookie_manager.expired_oauth_flow_cookie()),
        );
        response
    };

    if query.error.is_some() {
        tracing::warn!("Google OAuth provider returned an authorization error");
        return with_cleared_oauth_cookie(errors::json_error(
            StatusCode::BAD_REQUEST,
            "Google authorization was denied",
        ));
    }

    let code = match query.code {
        Some(code) => code,
        None => {
            tracing::warn!("Google OAuth callback missing authorization code");
            return with_cleared_oauth_cookie(errors::json_error(
                StatusCode::BAD_REQUEST,
                "Missing Google authorization code",
            ));
        }
    };
    let returned_state = match query.state {
        Some(state) => state,
        None => {
            tracing::warn!("Google OAuth callback missing state parameter");
            return with_cleared_oauth_cookie(errors::json_error(
                StatusCode::BAD_REQUEST,
                "Missing OAuth state",
            ));
        }
    };
    let (expected_state, pkce_verifier, should_connect) = match cookie_manager.oauth_flow(&headers)
    {
        Some(flow) => flow,
        None => {
            tracing::warn!("Missing or expired OAuth flow cookie");
            return with_cleared_oauth_cookie(errors::json_error(
                StatusCode::BAD_REQUEST,
                "Missing OAuth session",
            ));
        }
    };
    if returned_state != expected_state {
        tracing::warn!("Google OAuth state mismatch");
        return with_cleared_oauth_cookie(errors::json_error(
            StatusCode::BAD_REQUEST,
            "Invalid OAuth state",
        ));
    }

    let login = match auth_service
        .complete_google_login(code, pkce_verifier, should_connect)
        .await
    {
        Ok(login) => login,
        Err(error) => return with_cleared_oauth_cookie(errors::auth(error)),
    };

    tracing::info!(session_id = %login.session_id, "Google authentication successful");

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

pub async fn me(
    AuthenticatedUser { user }: AuthenticatedUser,
    State(state): State<AppState>,
) -> Response {
    let auth_service = AuthService::new(&state.db, state.config.clone());
    let gmail_status = auth_service
        .get_gmail_connection_status(user.id)
        .await
        .unwrap_or(None);

    let mut response = MeResponse::from(user);
    response.gmail_connected = gmail_status.map(|s| s.0);
    response.gmail_connected_at = gmail_status.map(|s| s.1);

    Json(response).into_response()
}

pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let cookie_manager = cookies::CookieManager::new(state.config.clone());

    if let Some(session_id) = cookie_manager.session_id(&headers) {
        let auth_service = AuthService::new(&state.db, state.config.clone());

        if let Err(err) = auth_service.logout(session_id).await {
            return errors::auth(err);
        }
        tracing::info!(session_id = %session_id, "User logged out");
    }

    let mut response = StatusCode::NO_CONTENT.into_response();

    response.headers_mut().insert(
        header::SET_COOKIE,
        cookies::cookie_header(&cookie_manager.expired_session_cookie()),
    );

    response
}
