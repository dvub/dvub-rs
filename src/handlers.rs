use std::{path::Path, sync::Arc};

use axum::{
    extract::{Path as AxumPath, State},
    response::Html,
};
use tera::Context;

use crate::{get_posts, AppState};

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

pub async fn posts(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("posts", &get_posts(Path::new("templates/posts")));

    Html(state.tera.render("posts.html", &context).unwrap())
}
