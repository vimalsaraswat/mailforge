use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::services::errors::AuthServiceError;

#[derive(Debug, Error)]
pub enum EmailTemplateError {
    #[error("database operation failed")]
    Database(#[from] sqlx::Error),
    #[error("email template not found")]
    NotFound,
}

impl IntoResponse for EmailTemplateError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Email template not found"),
            Self::Database(error) => {
                tracing::error!(?error, "email template request failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Email template storage failed",
                )
            }
        };

        (status, message).into_response()
    }
}

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
