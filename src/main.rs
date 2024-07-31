use std::{
    fs::{read_dir, read_to_string},
    path::Path,
    sync::Arc,
};

use axum::{
    extract::{Path as AxumPath, State},
    response::Html,
    routing::get,
    Router,
};
use notify::{RecursiveMode, Watcher};
use scraper::{Html as ScraperHtml, Selector};

use tera::{Context, Tera};
use tokio::net::TcpListener;
use tower_livereload::LiveReloadLayer;

struct AppState {
    tera: Tera,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState {
        tera: {
            let mut tera = match Tera::new("templates/**/*.html") {
                Ok(t) => t,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };
            // i dont know what this does
            tera.autoescape_on(vec![".html"]);

            tera
        },
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/posts", get(posts))
        .route("/posts/:post", get(render_post))
        // serve assets directory for compiled tailwind CSS
        .nest_service("/assets", tower_http::services::ServeDir::new("assets"))
        .with_state(state)
        .layer(configure_live_reload());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn render_post(
    AxumPath(r): AxumPath<String>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let context = Context::new();

    Html(
        state
            .tera
            .render(&format!("posts/{}", r), &context)
            .unwrap(),
    )
}

async fn root(State(state): State<Arc<AppState>>) -> Html<String> {
    let context = Context::new();
    Html(state.tera.render("index.html", &context).unwrap())
}

async fn posts(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut context = Context::new();
    context.insert("posts", &get_posts(Path::new("templates/posts")));

    Html(state.tera.render("posts.html", &context).unwrap())
}

fn configure_live_reload() -> LiveReloadLayer {
    let livereload = tower_livereload::LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let mut watcher = notify::recommended_watcher(move |_| reloader.reload()).unwrap();
    watcher
        .watch(Path::new("assets/"), RecursiveMode::Recursive)
        .unwrap();
    watcher
        .watch(Path::new("templates"), RecursiveMode::Recursive)
        .unwrap();
    livereload
}
#[derive(serde::Serialize)]
struct Post {
    href: String,
    title: String,
    description: String,
}

fn get_posts(path: &Path) -> Vec<Post> {
    let posts_dir = read_dir(path).unwrap();

    let mut posts_vec = Vec::new();

    for entry in posts_dir {
        let path = entry.unwrap().path();
        let href = &path.file_name().unwrap().to_str().unwrap();
        let str = read_to_string(&path).unwrap();

        let html = ScraperHtml::parse_fragment(&str);

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
            href: format!("posts/{}", href.to_string()),
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
