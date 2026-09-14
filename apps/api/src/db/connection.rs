use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing::info;

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    info!("Initializing PostgreSQL connection pool");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;
    info!("PostgreSQL connection pool established");
    Ok(pool)
}
