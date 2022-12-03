use crate::domain::articles::types::article::Article;

pub fn get_mock_articles() -> Box<[Article]> {
    // TODO: Add comments
    // let comments = get_mock_comments();

    Box::new([
        Article {
            id: "1".to_string(),
            title: "This is my first article".to_string(),
            content: "Yo, never thought this would happen! I'm actually writing an article.".to_string(),
            comments: Box::new([]),
        },
        Article {
            id: "2".to_string(),
            title: "Article no 2".to_string(),
            content: "Ayyyyy supp m8 lmao".to_string(),
            comments: Box::new([]),
        },
        Article {
            id: "3".to_string(),
            title: "One last for the run".to_string(),
            content: "This is it folks, wrap it up.".to_string(),
            comments: Box::new([]),
        },
    ])
}