use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;
use crate::domain::articles::api::get_articles;

#[function_component(ArticlesPage)]
pub fn articles_page() -> Html {
    let articles = get_articles();

    html! {
        <>
            <h2>{ "Articles" }</h2>
            <ul>
                { for articles.into_iter().map(|article| {
                    html! {
                        <li>
                            <Link<AppRoute> to={AppRoute::Article { id: article.id }}>
                                { article.title }
                            </Link<AppRoute>>
                        </li>
                    }
                })}
            </ul>
        </>
    }
}