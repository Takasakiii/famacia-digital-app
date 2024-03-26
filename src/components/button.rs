use yew::{classes, function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub text: &'static str,
    #[prop_or_default]
    pub color: Option<&'static str>,
    #[prop_or_default]
    pub full_width: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let full_width = if props.full_width {
        Some("is-fullwidth")
    } else {
        None
    };
    let classes = classes!("button", full_width, props.color);

    html! {
        <button class={classes}>{props.text}</button>
    }
}
