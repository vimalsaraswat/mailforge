use serde::Serialize;
use uuid::Uuid;

use crate::models::User;

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

impl From<User> for MeResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            picture: user.picture,
        }
    }
}
