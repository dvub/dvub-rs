use std::path::Path;

use axum::{response::Html, routing::get, Router};
use notify::{RecursiveMode, Watcher};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let livereload = tower_livereload::LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let mut watcher = notify::recommended_watcher(move |_| reloader.reload()).unwrap();
    watcher
        .watch(Path::new("assets/"), RecursiveMode::Recursive)
        .unwrap();
    watcher
        .watch(Path::new("pages/"), RecursiveMode::Recursive)
        .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/posts", get(posts))
        // serve assets directory for compiled tailwind CSS
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .layer(livereload);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<&'static str> {
    Html(std::include_str!("../pages/index.html"))
}

async fn posts() -> Html<&'static str> {
    Html(std::include_str!("../pages/posts.html"))
}
