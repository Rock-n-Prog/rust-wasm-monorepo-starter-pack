use yew::prelude::*;

use crate::domain::articles::mocks::articles::get_mock_articles;

#[function_component(ArticlesPage)]
pub fn articles_page() -> Html {
    let articles = get_mock_articles();

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