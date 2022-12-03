use super::types::Article;
use super::mocks::get_mock_articles;

pub fn get_articles() -> Vec<Article> {
    return get_mock_articles();
}

pub fn get_article(_id: String) -> Article {
    // TODO: get_mock_articles with filter
    Article {
        id: "3".to_string(),
        title: "One last for the run".to_string(),
        content: "This is it folks, wrap it up.".to_string(),
        comments: Vec::from([]),
    }
}
