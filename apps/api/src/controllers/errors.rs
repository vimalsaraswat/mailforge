use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::services::errors::AuthServiceError;

pub fn auth(error: AuthServiceError) -> Response {
    tracing::error!(?error, "authentication request failed");

    let (status, message) = match error {
        AuthServiceError::NotConfigured => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Google OAuth is not configured".to_string(),
        ),
        AuthServiceError::UnverifiedEmail | AuthServiceError::MissingRefreshToken => {
            (StatusCode::BAD_REQUEST, error.to_string())
        }
        AuthServiceError::Provider(_) => (
            StatusCode::BAD_GATEWAY,
            "Google authentication failed".to_string(),
        ),
        AuthServiceError::Database(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication storage failed".to_string(),
        ),
    };

    (status, message).into_response()
}
