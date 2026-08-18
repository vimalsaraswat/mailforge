use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
};

use crate::{
    http::cookies::CookieManager,
    models::User,
    repositories::{session::SessionRepository, user::UserRepository},
    state::AppState,
};

pub struct AuthenticatedUser {
    pub user: User,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let db = &state.db;

        let session_id = CookieManager::new(state.config.clone())
            .session_id(&parts.headers)
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let session = SessionRepository::new(db.clone())
            .find_valid(session_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let user = UserRepository::new(db.clone())
            .find_by_id(session.user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(Self { user })
    }
}
