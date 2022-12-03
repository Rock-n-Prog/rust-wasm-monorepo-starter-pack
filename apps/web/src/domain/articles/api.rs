use super::types::Article;
use super::mocks::get_mock_articles;

pub fn get_articles() -> Vec<Article> {
    return get_mock_articles();
}