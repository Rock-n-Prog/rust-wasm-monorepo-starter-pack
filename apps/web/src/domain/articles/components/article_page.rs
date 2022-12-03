use yew::prelude::*;
use crate::domain::articles::api::get_article;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub id: String,
}

// TODO: Stylize page
#[function_component(ArticlePage)]
pub fn article_page(props: &Props) -> Html {
    let article = get_article(props.id.clone());

    html! {
        <>
            <h2>{ "Article" }</h2>
            <p>{ article.title }</p>
        </>
    }
}