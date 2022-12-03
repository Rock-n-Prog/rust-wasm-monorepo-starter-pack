use super::types::Article;
use super::mocks::{get_mock_articles, get_mock_article};

// TODO: Make all of those async (https://github.com/jetli/rust-yew-realworld-example-app/blob/master/crates/conduit-wasm/src/routes/article/mod.rs)

pub fn get_articles() -> Vec<Article> {
    get_mock_articles()
}

pub fn get_article(id: String) -> Option<Article> {
    get_mock_article(id)
}
