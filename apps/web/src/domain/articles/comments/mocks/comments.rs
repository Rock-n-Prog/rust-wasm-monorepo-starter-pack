use crate::domain::articles::comments::types::comment::Comment;

pub fn get_mock_comments() -> Box<[Comment]> {
    Box::new([
        Comment {
            id: "1".to_string(),
            text: "Wow!".to_string(),
        },
        Comment {
            id: "2".to_string(),
            text: "Super!".to_string(),
        },
        Comment {
            id: "3".to_string(),
            text: "So cool!".to_string(),
        },
    ])
}