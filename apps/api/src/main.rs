use api::{config::Config, db, routes, state::AppState};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let address = config.address();

    let db = db::connection::connect(&config.database_url).await?;
    db::migration::migrate(&db).await?;

    let state = AppState { config, db };
    let app = routes::router(state);

    tracing::info!("Server listening on http://{}", address);
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
