use yew::{classes, function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct TitleProps {
    pub title: &'static str,
    #[prop_or_default]
    pub center: bool,
}

#[function_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let center = if props.center {
        Some("has-text-centered")
    } else {
        None
    };
    let classes = classes!("title", center);

    html! {
        <h1 class={classes}>{props.title}</h1>
    }
}
