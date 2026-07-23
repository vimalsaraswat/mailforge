use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct GoogleToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}
