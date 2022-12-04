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
    // TODO: List and ListItem components
    html! {
        <ul>
            { for props.articles.clone().into_iter().map(|article| {
                html! {
                    <Link<AppRoute> to={AppRoute::Article { id: article.id.clone() }}>
                        <li>
                            { article.title.clone() }
                        </li>
                    </Link<AppRoute>>
                }
            })}
        </ul>
    }
}