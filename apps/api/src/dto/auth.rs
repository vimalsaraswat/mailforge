use serde::Serialize;
use uuid::Uuid;

use crate::models::User;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
    pub gmail_connected: Option<bool>,
    pub gmail_connected_at: Option<DateTime<Utc>>,
}

impl From<User> for MeResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            picture: user.picture,
            gmail_connected: None,
            gmail_connected_at: None,
        }
    }
}
