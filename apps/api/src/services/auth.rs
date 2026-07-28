use sqlx::PgPool;

use crate::clients::google::oauth::GoogleOAuthClient;
use crate::config::Config;
use crate::repositories::mail_account::MailAccountRepository;
use crate::repositories::session::SessionRepository;
use crate::repositories::user::UserRepository;

pub struct AuthService {
    users: UserRepository,
    mail_accounts: MailAccountRepository,
    sessions: SessionRepository,
    oauth: GoogleOAuthClient,
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
        }
    }
}
