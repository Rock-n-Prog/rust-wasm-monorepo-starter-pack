use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;
use crate::domain::articles::api::get_articles;

#[function_component(ArticlesPage)]
pub fn articles_page() -> Html {
    let result = {
        use_async_with_options(
            async move { get_articles().await },
            UseAsyncOptions::enable_auto(),
        )
    };

    // TODO: Mega cleanup
    // TODO: Loading component
    // TODO: Error component
    html! {
        <>
            <h2>{ "Articles" }</h2>
            {
                if result.loading {
                    html! { "Loading" }
                } else {
                    html! {}
                }
            }
            {
                if let Some(articles) = &result.data {
                    html! {
                        <ul>
                            { for articles.into_iter().map(|article| {
                                html! {
                                    <li>
                                        <Link<AppRoute> to={AppRoute::Article { id: article.id.clone() }}>
                                            { article.title.clone() }
                                        </Link<AppRoute>>
                                    </li>
                                }
                            })}
                        </ul>
                    }
                } else {
                    html! {}
                }
            }
            {
                if let Some(error) = &result.error {
                    html! { error }
                } else {
                    html! {}
                }
            }
        </>
    }
}