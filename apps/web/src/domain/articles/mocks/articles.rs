use crate::domain::articles::comments::mocks::comments::get_mock_comments;
use crate::domain::articles::types::article::Article;

pub fn get_mock_articles() -> Vec<Article> {
    let comments = get_mock_comments();

    Vec::from([
        Article {
            id: "1".to_string(),
            title: "This is my first article".to_string(),
            content: "Yo, never thought this would happen! I'm actually writing an article.".to_string(),
            comments: comments.clone().into_iter().filter(|c| c.id == "1" || c.id == "2").collect(),
        },
        Article {
            id: "2".to_string(),
            title: "Article no 2".to_string(),
            content: "Ayyyyy supp m8 lmao".to_string(),
            comments: comments.clone().into_iter().filter(|c| c.id == "3").collect(),
        },
        Article {
            id: "3".to_string(),
            title: "One last for the run".to_string(),
            content: "This is it folks, wrap it up.".to_string(),
            comments: Vec::from([]),
        },
    ])
}