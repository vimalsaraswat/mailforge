use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Session;

#[derive(Clone)]
pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>, sqlx::Error> {
        sqlx::query_as::<_, Session>(
            r#"
            SELECT *
            FROM sessions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        expires_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<Session, sqlx::Error> {
        let id = Uuid::new_v4();

        sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (
                id,
                user_id,
                expires_at
            )
            VALUES (
                $1, $2, $3
            )
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            DELETE FROM sessions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_by_user(&self, user_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM sessions
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn find_valid(&self, id: Uuid) -> Result<Option<Session>, sqlx::Error> {
        sqlx::query_as::<_, Session>(
            r#"
            SELECT *
            FROM sessions
            WHERE id = $1
              AND expires_at > NOW()
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }
}
