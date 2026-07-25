use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::services::errors::AuthServiceError;

pub fn auth(error: AuthServiceError) -> Response {
    tracing::error!(?error, "authentication request failed");
    match error {
        AuthServiceError::NotConfigured => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Google OAuth is not configured",
        )
            .into_response(),
        AuthServiceError::UnverifiedEmail | AuthServiceError::MissingRefreshToken => {
            (StatusCode::BAD_REQUEST, error.to_string()).into_response()
        }
        AuthServiceError::Provider(_) => {
            (StatusCode::BAD_GATEWAY, "Google authentication failed").into_response()
        }
        AuthServiceError::Database(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication storage failed",
        )
            .into_response(),
    }
}
