use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}
