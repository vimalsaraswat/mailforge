use sqlx::PgPool;
use uuid::Uuid;

use crate::models::User;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
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
        provider_user_id: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT *
            FROM users
            WHERE provider = $1
              AND provider_user_id = $2
            "#,
        )
        .bind(provider)
        .bind(provider_user_id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn create(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (
                id,
                provider,
                provider_user_id,
                email,
                name,
                picture,
                created_at,
                updated_at
            )
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8
            )
            "#,
        )
        .bind(user.id)
        .bind(&user.provider)
        .bind(&user.provider_user_id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(&user.picture)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE users
            SET
                email = $2,
                name = $3,
                picture = $4,
                updated_at = $5
            WHERE id = $1
            "#,
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.name)
        .bind(&user.picture)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
