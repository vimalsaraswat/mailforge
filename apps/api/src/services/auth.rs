use chrono::{Duration, Utc};
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeVerifier};
use reqwest::Url;
use sqlx::PgPool;
use uuid::Uuid;

use crate::clients::google::oauth::GoogleOAuthClient;
use crate::config::Config;
use crate::models::User;
use crate::repositories::mail_account::MailAccountRepository;
use crate::repositories::session::SessionRepository;
use crate::repositories::user::UserRepository;
use crate::services::errors::AuthServiceError;
use crate::services::results::LoginResult;

pub struct AuthService {
    users: UserRepository,
    mail_accounts: MailAccountRepository,
    sessions: SessionRepository,
    oauth: GoogleOAuthClient,
    session_ttl_seconds: u64,
}

impl AuthService {
    pub fn new(pool: &PgPool, config: Config) -> Self {
        Self {
            users: UserRepository::new(pool.clone()),
            mail_accounts: MailAccountRepository::new(pool.clone()),
            sessions: SessionRepository::new(pool.clone()),
            oauth: GoogleOAuthClient::new(
                config.google_client_id.clone(),
                config.google_client_secret.clone(),
                config.google_redirect_uri.clone(),
            ),
            session_ttl_seconds: config.session_ttl_seconds,
        }
    }

    pub async fn current_user(&self, session_id: Uuid) -> Result<Option<User>, AuthServiceError> {
        let Some(session) = self.sessions.find_by_id(session_id).await? else {
            return Ok(None);
        };
        if session.expires_at <= Utc::now() {
            return Ok(None);
        }
        Ok(self.users.find_by_id(session.user_id).await?)
    }

    pub fn start_google_login(&self) -> (Url, CsrfToken, PkceCodeVerifier) {
        self.oauth.authorization_url()
    }

    pub async fn complete_google_login(
        &self,
        code: String,
        pkce_verifier: String,
    ) -> Result<LoginResult, AuthServiceError> {
        let token = self
            .oauth
            .exchange_code(
                AuthorizationCode::new(code),
                PkceCodeVerifier::new(pkce_verifier),
            )
            .await
            .map_err(AuthServiceError::Provider)?;
        let profile = self
            .oauth
            .user_info(&token.access_token)
            .await
            .map_err(AuthServiceError::Provider)?;

        if !profile.verified_email {
            return Err(AuthServiceError::UnverifiedEmail);
        }

        let now = Utc::now();
        let user =
            if let Some(mut user) = self.users.find_by_provider("google", &profile.id).await? {
                user.email = profile.email.clone();
                user.name = profile.name.clone();
                user.picture = profile.picture.clone();
                user.updated_at = now;
                self.users.update(&user).await?;
                user
            } else {
                let user = User {
                    id: Uuid::new_v4(),
                    provider: "google".to_string(),
                    provider_user_id: profile.id.clone(),
                    email: profile.email.clone(),
                    name: profile.name.clone(),
                    picture: profile.picture.clone(),
                    created_at: now,
                    updated_at: now,
                };
                self.users.create(&user).await?;
                user
            };

        let existing_account = self
            .mail_accounts
            .find_by_provider("google", &profile.id)
            .await?;
        let refresh_token = token
            .refresh_token
            .or_else(|| {
                existing_account
                    .as_ref()
                    .map(|account| account.refresh_token.clone())
            })
            .ok_or(AuthServiceError::MissingRefreshToken)?;
        let expires_at = token.expires_at.unwrap_or_else(|| now + Duration::hours(1));

        if let Some(account) = existing_account {
            self.mail_accounts
                .update_tokens(
                    account.id,
                    &token.access_token,
                    &refresh_token,
                    expires_at,
                    now,
                )
                .await?;
        } else {
            self.mail_accounts
                .create(&crate::models::MailAccount {
                    id: Uuid::new_v4(),
                    user_id: user.id,
                    provider: "google".to_string(),
                    account_id: profile.id,
                    email: profile.email,
                    access_token: token.access_token,
                    refresh_token,
                    expires_at,
                    created_at: now,
                    updated_at: now,
                })
                .await?;
        }

        let session = self
            .sessions
            .create(&crate::models::Session {
                id: Uuid::new_v4(),
                user_id: user.id,
                expires_at: now + Duration::seconds(self.session_ttl_seconds as i64),
                created_at: now,
            })
            .await?;

        Ok(LoginResult {
            session_id: session.id,
            expires_at: session.expires_at,
        })
    }
}
