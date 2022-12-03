use yew::prelude::*;
use yew_hooks::prelude::*;
use crate::domain::articles::api::get_articles;
use super::article_list::ArticleList;

#[function_component(ArticlesPage)]
pub fn articles_page() -> Html {
    let result = {
        use_async_with_options(
            async move { get_articles().await },
            UseAsyncOptions::enable_auto(),
        )
    };

    html! {
        <>
            <h2>{ "Articles" }</h2>
            {
                if result.loading {
                    html! { "Loading" }
                } else if let Some(error) = &result.error {
                    html! { error }
                } else if let Some(articles) = &result.data {
                    html! { <ArticleList articles={articles.clone()} /> }
                } else {
                    html! { "Could not fetch articles" }
                }
            }
        </>
    }
}