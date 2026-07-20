use api::{config::Config, db, router, state::AppState};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env();
    let address = config.address();

    let db = db::connection::connect(&config.database_url).await?;
    db::migration::migrate(&db).await?;

    let state = AppState { config, db };

    let app = router::router(state);
    let listener = TcpListener::bind(address).await.unwrap();

    axum::serve(listener, app).await?;
    Ok(())
}
