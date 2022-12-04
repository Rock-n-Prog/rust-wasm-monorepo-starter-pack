use yew::prelude::*;
use crate::domain::articles::comments::types::Comment;
use crate::components::data::{list::List, list_item::ListItem};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub comments: Vec<Comment>,
}

#[function_component(CommentList)]
pub fn comment_list(props: &Props) -> Html {
    html! {
        <List>
            { for props.comments.clone().into_iter().map(|comment| {
                html! {
                    <ListItem>{ comment.text }</ListItem>
                }
            })}
        </List>
    }
}