use super::types::Article;
use super::mocks::{get_mock_articles, get_mock_article};

pub fn get_articles() -> Vec<Article> {
    get_mock_articles()
}

pub fn get_article(id: String) -> Option<Article> {
    get_mock_article(id)
}
