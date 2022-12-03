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

    // TODO: Can this else if logic be done with a match?
    // TODO: Loading component
    // TODO: Error component (including final else)
    html! {
        <>
            <h2>{ "Articles" }</h2>
            {
                if result.loading {
                    html! { "Loading" }
                } else if let Some(error) = &result.error {
                    html! { error }
                } else if let Some(articles) = &result.data {
                    html! {
                        <ul>
                            { for articles.iter().map(|article| {
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
                    html! { "Could not fetch articles" }
                }
            }
        </>
    }
}