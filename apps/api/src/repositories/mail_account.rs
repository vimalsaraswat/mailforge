use sqlx::PgPool;
use uuid::Uuid;

use crate::models::MailAccount;

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

    pub async fn create(&self, account: &MailAccount) -> Result<MailAccount, sqlx::Error> {
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
                expires_at,
                created_at,
                updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
            )
            RETURNING *
            "#,
        )
        .bind(account.id)
        .bind(account.user_id)
        .bind(&account.provider)
        .bind(&account.account_id)
        .bind(&account.email)
        .bind(&account.access_token)
        .bind(&account.refresh_token)
        .bind(account.expires_at)
        .bind(account.created_at)
        .bind(account.updated_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_tokens(
        &self,
        id: Uuid,
        access_token: &str,
        refresh_token: &str,
        expires_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<MailAccount, sqlx::Error> {
        sqlx::query_as::<_, MailAccount>(
            r#"
            UPDATE mail_accounts
            SET
                access_token = $2,
                refresh_token = $3,
                expires_at = $4,
                updated_at = $5
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(access_token)
        .bind(refresh_token)
        .bind(expires_at)
        .bind(updated_at)
        .fetch_one(&self.pool)
        .await
    }
}
