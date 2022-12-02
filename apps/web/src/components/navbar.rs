use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: String,
}

// TODO: Use theme values
#[styled_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    html! {
        <div class={css!(r#"
            padding: 8px 16px;
            display: flex;
            justify-content: center;

            @media only screen and (max-width: 767px) {
                justify-content: flex-start;
            }
        "#)}>
            <h1>{ props.title.clone() }</h1>
        </div>
    }
}
