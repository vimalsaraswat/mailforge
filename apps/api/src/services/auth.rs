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

    pub fn start_google_login(&self, include_gmail: bool) -> (Url, CsrfToken, PkceCodeVerifier) {
        self.oauth.authorization_url(include_gmail)
    }

    pub async fn complete_google_login(
        &self,
        code: String,
        pkce_verifier: String,
        should_connect: bool,
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

        if !profile.email_verified {
            return Err(AuthServiceError::UnverifiedEmail);
        }

        let user = self.sync_user(&profile).await?;

        if should_connect {
            self.sync_mail_account(&user, &profile, &token).await?;
        }

        let session = self.create_session(user.id).await?;

        Ok(LoginResult {
            session_id: session.id,
            expires_at: session.expires_at,
        })
    }

    async fn sync_user(&self, profile: &crate::clients::google::models::GoogleUserInfo) -> Result<User, AuthServiceError> {
        let now = Utc::now();
        if let Some(mut user) = self.users.find_by_provider("google", &profile.sub).await? {
            user.email = profile.email.clone();
            user.name = profile.name.clone();
            user.picture = profile.picture.clone();
            user.updated_at = now;
            self.users.update(&user).await?;
            Ok(user)
        } else {
            let user = User {
                id: Uuid::new_v4(),
                provider: "google".to_string(),
                provider_user_id: profile.sub.clone(),
                email: profile.email.clone(),
                name: profile.name.clone(),
                picture: profile.picture.clone(),
                created_at: now,
                updated_at: now,
            };
            self.users.create(&user).await?;
            Ok(user)
        }
    }

    async fn sync_mail_account(
        &self,
        user: &User,
        profile: &crate::clients::google::models::GoogleUserInfo,
        token: &crate::clients::google::models::GoogleToken,
    ) -> Result<(), AuthServiceError> {
        let now = Utc::now();
        let expires_at = token.expires_at.unwrap_or_else(|| now + Duration::hours(1));
        let existing_account = self
            .mail_accounts
            .find_by_provider("google", &profile.sub)
            .await?;

        if let Some(rt) = &token.refresh_token {
            if let Some(account) = existing_account {
                self.mail_accounts
                    .update_tokens(account.id, &token.access_token, rt, expires_at, now)
                    .await?;
            } else {
                self.mail_accounts
                    .create(&crate::models::MailAccount {
                        id: Uuid::new_v4(),
                        user_id: user.id,
                        provider: "google".to_string(),
                        account_id: profile.sub.clone(),
                        email: profile.email.clone(),
                        access_token: token.access_token.clone(),
                        refresh_token: rt.clone(),
                        expires_at,
                        created_at: now,
                        updated_at: now,
                    })
                    .await?;
            }
        }
        Ok(())
    }

    async fn create_session(&self, user_id: Uuid) -> Result<crate::models::Session, AuthServiceError> {
        let now = Utc::now();
        Ok(self
            .sessions
            .create(&crate::models::Session {
                id: Uuid::new_v4(),
                user_id,
                expires_at: now + Duration::seconds(self.session_ttl_seconds as i64),
                created_at: now,
            })
            .await?)
    }

    pub async fn logout(&self, session_id: Uuid) -> Result<(), AuthServiceError> {
        self.sessions.delete(session_id).await?;
        Ok(())
    }
}
