use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::EmailTemplate, repositories::email_template::EmailTemplateRepository};

pub struct EmailTemplateService {
    repository: EmailTemplateRepository,
}

impl EmailTemplateService {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            repository: EmailTemplateRepository::new(pool.clone()),
        }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        name: &str,
        subject: &str,
        body: &str,
    ) -> Result<EmailTemplate, sqlx::Error> {
        self.repository.create(user_id, name, subject, body).await
    }

    pub async fn get(&self, user_id: Uuid, id: Uuid) -> Result<Option<EmailTemplate>, sqlx::Error> {
        self.repository.find_by_id(id, user_id).await
    }

    pub async fn list(&self, user_id: Uuid) -> Result<Vec<EmailTemplate>, sqlx::Error> {
        self.repository.find_by_user(user_id).await
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        id: Uuid,
        name: &str,
        subject: &str,
        body: &str,
    ) -> Result<Option<EmailTemplate>, sqlx::Error> {
        self.repository
            .update(id, user_id, name, subject, body)
            .await
    }

    pub async fn delete(&self, user_id: Uuid, id: Uuid) -> Result<bool, sqlx::Error> {
        self.repository.delete(id, user_id).await
    }
}
