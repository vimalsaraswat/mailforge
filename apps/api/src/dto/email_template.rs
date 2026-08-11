use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::EmailTemplate;

#[derive(Debug, Deserialize)]
pub struct CreateEmailTemplateRequest {
    pub name: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEmailTemplateRequest {
    pub name: String,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct EmailTemplateResponse {
    pub id: Uuid,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<EmailTemplate> for EmailTemplateResponse {
    fn from(template: EmailTemplate) -> Self {
        Self {
            id: template.id,
            name: template.name,
            subject: template.subject,
            body: template.body,
            created_at: template.created_at,
            updated_at: template.updated_at,
        }
    }
}
