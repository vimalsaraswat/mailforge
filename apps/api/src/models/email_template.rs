use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct EmailTemplate {
    pub id: Uuid,
    pub user_id: Uuid,

    pub name: String,
    pub subject: String,
    pub body: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
