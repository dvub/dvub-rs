use std::{path::Path, sync::Arc};

use crate::{get_posts, AppState};
use axum::{
    extract::{Path as AxumPath, State},
    response::Html,
};
use tera::Context;

pub async fn render_post(
    AxumPath(r): AxumPath<String>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let context = Context::new();

    Html(
        state
            .tera
            .render(&format!("posts/{}.html", r), &context)
            .unwrap(),
    )
}

pub async fn root(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("posts", &get_posts(Path::new("templates/posts")));

    Html(state.tera.render("index.html", &context).unwrap())
}
