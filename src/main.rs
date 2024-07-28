use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on localhost 3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    println!("Serving root");

    "Hello, World!"
}
