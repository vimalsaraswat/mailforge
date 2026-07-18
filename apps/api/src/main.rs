use api::router;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, router::router()).await.unwrap();
}
