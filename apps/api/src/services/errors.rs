use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("Google OAuth is not configured")]
    NotConfigured,
    #[error("Google authentication failed")]
    Provider(#[source] anyhow::Error),
    #[error("database operation failed")]
    Database(#[source] sqlx::Error),
    #[error("Google email is not verified")]
    UnverifiedEmail,
    #[error("Google did not provide a refresh token")]
    MissingRefreshToken,
}

impl From<sqlx::Error> for AuthServiceError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}
