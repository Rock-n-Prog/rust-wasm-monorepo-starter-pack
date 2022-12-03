use super::types::Article;
use super::mocks::{get_mock_articles, get_mock_article};

pub async fn get_articles() -> Result<Vec<Article>, String> {
    Ok(get_mock_articles())
}

pub async fn get_article(id: String) -> Result<Option<Article>, String> {
    Ok(get_mock_article(id))
}
