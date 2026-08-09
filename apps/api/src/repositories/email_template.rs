use sqlx::PgPool;

use crate::models::EmailTemplate;

pub struct EmailTemplateRepository {
    pool: PgPool,
}

impl EmailTemplateRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, email_template: EmailTemplate) -> Result<EmailTemplate, sqlx::Error> {
        sqlx::query_as::<_, EmailTemplate>(
            r#"INSERT INTO email_templates (
                id,
                name,
                subject,
                body,
                user_id
            ) VALUES (
                $1,
                $2,
                $3,
                $4,
                $5
            ) RETURNING *"#
        )
            .bind(email_template.id)
            .bind(email_template.name)
            .bind(email_template.subject)
            .bind(email_template.body)
            .bind(email_template.user_id)
            .fetch_one(&self.pool)
            .await
    }
}
