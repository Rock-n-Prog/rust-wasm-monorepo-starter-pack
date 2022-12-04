use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[styled_component(List)]
pub fn list(props: &Props) -> Html {
    html! {
        <ul class={css!(
            r#"
                  list-style-type: none;
                  padding: 0;
                  margin: 0;
            "#,
        )}>
            { props.children.clone() }
        </ul>
    }
}
