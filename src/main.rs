use std::sync::Arc;

use axum::{routing::get, Router};

use htrx::{
    handlers::{render_post, root},
    AppState,
};

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(root))
        // uses a capture!
        .route("/posts/:post", get(render_post))
        // serve assets directory for CSS, JS, media, etc.
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .with_state(state);

    #[cfg(debug_assertions)]
    let app = {
        use notify::Watcher;
        let livereload = tower_livereload::LiveReloadLayer::new();
        let reloader = livereload.reloader();
        let mut watcher = notify::recommended_watcher(move |_| reloader.reload()).unwrap();
        watcher
            .watch(
                std::path::Path::new("assets"),
                notify::RecursiveMode::Recursive,
            )
            .unwrap();
        watcher
            .watch(
                std::path::Path::new("templates"),
                notify::RecursiveMode::Recursive,
            )
            .unwrap();
        println!("Reloading..");
        app.layer(livereload)
    };

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
