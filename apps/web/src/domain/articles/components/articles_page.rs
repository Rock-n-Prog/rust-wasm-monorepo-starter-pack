use yew::prelude::*;
use crate::domain::articles::api::get_articles;

#[function_component(ArticlesPage)]
pub fn articles_page() -> Html {
    let articles = get_articles();

    html! {
        <>
            <h2>{ "Articles" }</h2>
            { for articles.into_iter().map(|article| {
                html! {
                    <p>{ article.title }</p>
                }
            })}
        </>
    }
}