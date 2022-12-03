use yew::prelude::*;
use crate::domain::articles::comments::types::Comment;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub comments: Vec<Comment>,
}

#[function_component(CommentList)]
pub fn comment_list(props: &Props) -> Html {
    html! {
        <ul>
            { for props.comments.clone().into_iter().map(|comment| {
                html! {
                    <li>{ comment.text }</li>
                }
            })}
        </ul>
    }
}