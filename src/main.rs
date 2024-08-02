use std::sync::Arc;

use axum::{routing::get, Router};

use dvub_rs::{
    handlers::{render_post, root},
    AppState,
};
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
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

    Ok(app.into())
}
