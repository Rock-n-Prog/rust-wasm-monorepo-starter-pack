use crate::domain::articles::comments::types::comment::Comment;

#[derive(Debug, Clone, PartialEq)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub content: String,
    pub comments: Box<[Comment]>,
}