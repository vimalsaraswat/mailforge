use sqlx::PgPool;
use uuid::Uuid;

use crate::models::EmailTemplate;

pub struct EmailTemplateRepository {
    pool: PgPool,
}

impl EmailTemplateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        name: &str,
        subject: &str,
        body: &str,
    ) -> Result<EmailTemplate, sqlx::Error> {
        let id = Uuid::new_v4();

        sqlx::query_as::<_, EmailTemplate>(
            r#"
            INSERT INTO email_templates (
                id,
                user_id,
                name,
                subject,
                body
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(name)
        .bind(subject)
        .bind(body)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_id(
        &self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<EmailTemplate>, sqlx::Error> {
        sqlx::query_as::<_, EmailTemplate>(
            r#"
            SELECT *
            FROM email_templates
            WHERE id = $1
              AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<EmailTemplate>, sqlx::Error> {
        sqlx::query_as::<_, EmailTemplate>(
            r#"
            SELECT *
            FROM email_templates
            WHERE user_id = $1
            ORDER BY updated_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn update(
        &self,
        id: Uuid,
        user_id: Uuid,
        name: &str,
        subject: &str,
        body: &str,
    ) -> Result<Option<EmailTemplate>, sqlx::Error> {
        sqlx::query_as::<_, EmailTemplate>(
            r#"
            UPDATE email_templates
            SET
                name = $3,
                subject = $4,
                body = $5,
                updated_at = NOW()
            WHERE id = $1
              AND user_id = $2
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(name)
        .bind(subject)
        .bind(body)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn delete(&self, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM email_templates
            WHERE id = $1
              AND user_id = $2
            "#,
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
