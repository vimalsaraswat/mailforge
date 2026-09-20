use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

use crate::services::errors::AuthServiceError;

#[derive(Debug, Error)]
pub enum EmailTemplateError {
    #[error("database operation failed")]
    Database(#[from] sqlx::Error),
    #[error("email template not found")]
    NotFound,
    #[error("validation error: {0}")]
    Validation(String),
}

impl IntoResponse for EmailTemplateError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Email template not found".to_string()),
            Self::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::Database(error) => {
                tracing::error!(?error, "Email template database operation failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Email template storage failed".to_string(),
                )
            }
        };

        json_error(status, message)
    }
}

pub fn json_error(status: StatusCode, message: impl Into<String>) -> Response {
    (status, Json(json!({ "error": message.into() }))).into_response()
}

pub fn auth(error: AuthServiceError) -> Response {
    tracing::error!(?error, "Authentication request failed");

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

    json_error(status, message)
}
