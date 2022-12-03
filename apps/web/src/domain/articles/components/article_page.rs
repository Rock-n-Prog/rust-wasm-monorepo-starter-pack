use yew::prelude::*;
use crate::domain::articles::api::get_article;
use crate::domain::not_found::components::not_found_page::NotFoundPage;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub id: String,
}

#[function_component(ArticlePage)]
pub fn article_page(props: &Props) -> Html {
    let article = get_article(props.id.clone());

    // TODO: Move article + comments to its own component
    match article {
        Some(article) => html! {
            <>
                <h2>{ article.title }</h2>
                <p>{ article.content }</p>
                <h3>{ "Comments" }</h3>
                { if article.comments.is_empty() {
                    html! { <p>{ "No comments!" }</p> }
                } else {
                    html! {
                        <ul>
                            { for article.comments.into_iter().map(|comment| {
                                html! {
                                    <li>{ comment.text }</li>
                                }
                            })}
                        </ul>
                    }
                }}
            </>
        },
        None => html! { <NotFoundPage /> },
    }
}