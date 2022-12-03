use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::components::feedback::{alert::{Alert, Severity}, loading_spinner::LoadingSpinner};
use crate::domain::articles::api::get_article;
use crate::domain::articles::comments::components::comment_list::CommentList;
use crate::domain::not_found::components::not_found_page::NotFoundPage;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub id: String,
}

#[function_component(ArticlePage)]
pub fn article_page(props: &Props) -> Html {
    let result = {
        let id = props.id.clone();
        use_async_with_options(
            async move { get_article(id).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    html! {
        <>
            {
                if result.loading {
                    html! { <LoadingSpinner /> }
                } else if let Some(error) = &result.error {
                    html! { <Alert text={error.clone()} severity={Severity::Error} /> }
                } else if let Some(article) = &result.data {
                    match article {
                        Some(article) => html! {
                            <>
                                <h2>{ article.title.clone() }</h2>
                                <p>{ article.content.clone() }</p>
                                <h3>{ "Comments" }</h3>
                                { if article.comments.is_empty() {
                                    html! { <p>{ "No comments!" }</p> }
                                } else {
                                    html! { <CommentList comments={article.comments.clone()} /> }
                                }}
                            </>
                        },
                        None => html! { <NotFoundPage /> },
                    }
                } else {
                    html! { <Alert text="Could not fetch article" severity={Severity::Error} /> }
                }
            }
        </>
    }
}