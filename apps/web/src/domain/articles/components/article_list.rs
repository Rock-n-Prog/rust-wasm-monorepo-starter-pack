use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::AppRoute;
use crate::domain::articles::types::Article;
use crate::components::data::{list::List, list_item::ListItem};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub articles: Vec<Article>,
}

#[function_component(ArticleList)]
pub fn article_list(props: &Props) -> Html {
    html! {
        <List>
            { for props.articles.clone().into_iter().map(|article| {
                html! {
                    <Link<AppRoute> to={AppRoute::Article { id: article.id.clone() }}>
                        <ListItem>
                            { article.title }
                        </ListItem>
                    </Link<AppRoute>>
                }
            })}
        </List>
    }
}