use std::{path::Path, sync::Arc};

use axum::{routing::get, Router};

use htrx::{
    handlers::{render_post, root},
    AppState,
};

use notify::RecursiveMode;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // the tera docs just say to use lazy_static for tera's template state
    // but axum has a whole built-in thing for state so we should really use that
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(root))
        // uses a capture!
        .route("/posts/:post", get(render_post))
        // serve assets directory for CSS, JS, media, etc.
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .with_state(state);

    // only configure live reload if we're debugging (not in releases!)
    #[cfg(debug_assertions)]
    let app = {
        use notify::Watcher;
        let livereload = tower_livereload::LiveReloadLayer::new();
        let reloader = livereload.reloader();

        let mut watcher = notify::recommended_watcher(move |_| reloader.reload()).unwrap();
        // watch the directories of interest
        watcher
            .watch(Path::new("assets"), RecursiveMode::Recursive)
            .unwrap();
        watcher
            .watch(Path::new("templates"), RecursiveMode::Recursive)
            .unwrap();
        println!("Reloading..");
        // of course, add our live reloading to our app
        app.layer(livereload)
    };

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    // boom!
    axum::serve(listener, app).await.unwrap();
}
