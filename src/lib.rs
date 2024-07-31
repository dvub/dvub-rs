pub mod handlers;

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

pub fn get_posts(path: &Path) -> Vec<Post> {
    let posts_dir = read_dir(path).unwrap();

    let mut posts_vec = Vec::new();

    for entry in posts_dir {
        let path = entry.unwrap().path();
        let href = &path.file_stem().unwrap().to_str().unwrap();
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
        let results = get_posts(Path::new("./test-templates"));

        assert_eq!(results.len(), 1); // make sure theres only 1 post read from the 1 file
        assert_eq!(results[0].title, "Test");
        assert_eq!(results[0].description, "Cool stuff here!");
    }
}
