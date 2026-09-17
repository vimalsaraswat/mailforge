use sqlx::PgPool;
use uuid::Uuid;

use crate::models::MailAccount;

#[derive(Debug, Clone)]
pub struct NewMailAccount<'a> {
    pub user_id: Uuid,
    pub provider: &'a str,
    pub account_id: &'a str,
    pub email: &'a str,
    pub access_token: &'a str,
    pub refresh_token: &'a str,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct MailAccountRepository {
    pool: PgPool,
}

impl MailAccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<MailAccount>, sqlx::Error> {
        sqlx::query_as::<_, MailAccount>(
            r#"
            SELECT *
            FROM mail_accounts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_provider(
        &self,
        provider: &str,
        account_id: &str,
    ) -> Result<Option<MailAccount>, sqlx::Error> {
        sqlx::query_as::<_, MailAccount>(
            r#"
            SELECT *
            FROM mail_accounts
            WHERE provider = $1
              AND account_id = $2
            "#,
        )
        .bind(provider)
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<MailAccount>, sqlx::Error> {
        sqlx::query_as::<_, MailAccount>(
            r#"
            SELECT *
            FROM mail_accounts
            WHERE user_id = $1
            ORDER BY created_at ASC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create(&self, params: NewMailAccount<'_>) -> Result<MailAccount, sqlx::Error> {
        let id = Uuid::new_v4();

        sqlx::query_as::<_, MailAccount>(
            r#"
            INSERT INTO mail_accounts (
                id,
                user_id,
                provider,
                account_id,
                email,
                access_token,
                refresh_token,
                expires_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8
            )
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(params.user_id)
        .bind(params.provider)
        .bind(params.account_id)
        .bind(params.email)
        .bind(params.access_token)
        .bind(params.refresh_token)
        .bind(params.expires_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_tokens(
        &self,
        id: Uuid,
        access_token: &str,
        refresh_token: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<MailAccount, sqlx::Error> {
        sqlx::query_as::<_, MailAccount>(
            r#"
            UPDATE mail_accounts
            SET
                access_token = $2,
                refresh_token = $3,
                expires_at = $4,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_access_token(
        &self,
        id: Uuid,
        access_token: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<MailAccount, sqlx::Error> {
        sqlx::query_as::<_, MailAccount>(
            r#"
            UPDATE mail_accounts
            SET
                access_token = $2,
                expires_at = $3,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(access_token)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await
    }
}
