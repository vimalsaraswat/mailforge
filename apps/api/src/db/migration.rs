use sqlx::PgPool;
use tracing::info;

pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("Running pending database migrations");
    sqlx::migrate!("./migrations").run(pool).await?;
    info!("Database migrations applied successfully");
    Ok(())
}
