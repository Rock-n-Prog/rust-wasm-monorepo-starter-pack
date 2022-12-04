use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::inputs::button::Button;
use crate::components::typography::heading_1::Heading1;
use crate::styles::theme::hooks::use_theme_context::use_theme_context;
use crate::routes::AppRoute;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub title: String,
}

#[styled_component(Header)]
pub fn header(props: &Props) -> Html {
    let theme_context = use_theme_context();

    let onclick = Callback::from(move |_| theme_context.switch_theme.emit(()));

    // TODO: add class to remove margin
    // <Heading1 class={css!("margin: 0;")}>{ props.title.clone() }</Heading1>
    html! {
        <div class={css!(
            r#"
                padding: ${spacing_m} ${spacing_l};
                display: flex;
                align-items: center;
                justify-content: space-between;
                gap: ${spacing_m};
                text-align: center;
                background-color: ${surface};

                @media only screen and (max-width: ${breakpoint_s}) {
                    justify-content: center;
                    flex-direction: column;
                }
            "#,
            spacing_m = theme_context.theme.spacings.m.clone(),
            spacing_l = theme_context.theme.spacings.l.clone(),
            breakpoint_s = theme_context.theme.breakpoints.s.clone(),
            surface = theme_context.theme.colors.surface.clone(),
        )}>
            <Link<AppRoute> to={AppRoute::Home}>
                <Heading1>{ props.title.clone() }</Heading1>
            </Link<AppRoute>>
            <Button {onclick}>{ "Switch theme" }</Button>
        </div>
    }
}
