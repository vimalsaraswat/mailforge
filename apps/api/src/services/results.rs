use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct LoginResult {
    pub session_id: Uuid,
    pub expires_at: DateTime<Utc>,
}
