use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Session {
    pub id: Uuid,

    pub user_id: Uuid,

    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
