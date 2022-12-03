use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;
use crate::domain::articles::types::Article;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub articles: Vec<Article>,
}

#[function_component(ArticleList)]
pub fn article_list(props: &Props) -> Html {
    html! {
        <ul>
            { for props.articles.clone().into_iter().map(|article| {
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
}