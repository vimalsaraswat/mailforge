use api::{config::Config, router};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let listener = TcpListener::bind(config.address()).await.unwrap();

    axum::serve(listener, router::router()).await.unwrap();
}
