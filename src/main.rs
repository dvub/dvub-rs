use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};

use askama::Template;
use axum::{routing::get, Router};
use notify::{RecursiveMode, Watcher};
use scraper::{Html, Selector};
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
        .watch(Path::new("templates"), RecursiveMode::Recursive)
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

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate {}

#[derive(Template)]
#[template(path = "posts.html")]
struct PostsTemplate {
    posts: Vec<Post>,
}

struct Post {
    title: String,
    description: String,
}

async fn root() -> HomeTemplate {
    HomeTemplate {}
}

async fn posts() -> PostsTemplate {
    let posts = get_posts(Path::new("./templates/posts"));

    PostsTemplate { posts }
}

fn get_posts(path: &Path) -> Vec<Post> {
    let posts_dir = read_dir(path).unwrap();

    let mut posts_vec = Vec::new();

    for entry in posts_dir {
        let path = entry.unwrap().path();
        let str = read_to_string(path).unwrap();

        let html = Html::parse_fragment(&str);

        let title_selector = Selector::parse("title").unwrap();
        let meta_selector = Selector::parse("meta").unwrap();

        let mut title_results = html.select(&title_selector);
        let meta_results = html.select(&meta_selector);

        let title = title_results.nth(0).unwrap().inner_html();

        let mut description = "";

        for n in meta_results {
            if n.attr("name") == Some("description") {
                description = n.attr("content").unwrap();
            }
        }
        posts_vec.push(Post {
            title,
            description: description.to_owned(),
        });
    }
    posts_vec
}

#[cfg(test)]
mod tests {
    use crate::get_posts;
    use std::path::Path;

    #[test]
    fn test_read_metadata() {
        let results = get_posts(Path::new("./templates/test"));

        assert_eq!(results.len(), 1);

        assert_eq!(results[0].title, "Test");
        assert_eq!(results[0].description, "Cool stuff here!");
    }
}
