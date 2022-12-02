use stylist::yew::{styled_component, Global};
use yew::prelude::*;

// TODO: Implement theming: https://github.com/futursolo/stylist-rs/tree/master/examples/yew-theme-context

// TODO: Modify global styles
#[styled_component(GlobalStyles)]
pub fn global_styles() -> Html {
    html! {
        <Global css={css!(r#"
            html, body {
                font-family: sans-serif;
                padding: 0;
                margin: 0;
                min-height: 100vh;
                background-color: rgb(237, 244, 255);
            }
        "#)} />
    }
}