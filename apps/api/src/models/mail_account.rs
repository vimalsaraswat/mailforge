use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct MailAccount {
    pub id: Uuid,

    pub user_id: Uuid,

    pub provider: String,

    pub account_id: String,
    pub email: String,

    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
