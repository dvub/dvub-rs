pub mod handlers;

use axum::{http::StatusCode, response::IntoResponse};
use scraper::{Html as ScraperHtml, Selector};
use std::{
    fs::{read_dir, read_to_string},
    path::Path,
};
use tera::Tera;

pub struct AppState {
    tera: Tera,
}

#[derive(serde::Serialize)]
pub struct Post {
    href: String,
    title: String,
    description: String,
}

pub struct AppError(anyhow::Error);
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

impl AppState {
    // TODO:
    // parameterize new() with template path
    pub fn new() -> Self {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // i dont know what this does
        tera.autoescape_on(vec![".html"]);

        AppState { tera }
    }
}

pub fn get_posts(path: &Path) -> anyhow::Result<Vec<Post>> {
    let posts_dir = read_dir(path)?;
    let mut posts_vec = Vec::new();

    for entry in posts_dir {
        let path = entry?.path();

        // TODO:
        // deal with unwraps
        let href = &path.file_stem().unwrap().to_str().unwrap();
        let file_contents = read_to_string(&path)?;

        let html = ScraperHtml::parse_fragment(&file_contents);

        // TODO:
        // figure out what to do with these unwraps
        let title_selector = Selector::parse("title").unwrap();
        let meta_selector = Selector::parse("meta").unwrap();

        let mut title_results = html.select(&title_selector);
        let meta_results = html.select(&meta_selector);

        let title = title_results
            .nth(0)
            .expect("No title element found.")
            .inner_html();

        let mut description = "No description found..";

        for meta in meta_results {
            if meta.attr("name") == Some("description") {
                description = meta.attr("content").expect("description had no content");
            }
        }
        posts_vec.push(Post {
            href: format!("posts/{}", href.to_string()),
            title,
            description: description.to_owned(),
        });
    }
    Ok(posts_vec)
}

#[cfg(test)]
mod tests {
    use crate::get_posts;
    use std::path::Path;

    #[test]
    fn test_read_metadata() {
        let results = get_posts(Path::new("./test-templates")).unwrap();

        assert_eq!(results.len(), 1); // make sure theres only 1 post read from the 1 file
        assert_eq!(results[0].title, "Test");
        assert_eq!(results[0].description, "Cool stuff here!");
    }
}
