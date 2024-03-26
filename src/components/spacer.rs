use crate::utils::get_inline_style;
use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct SpacerProps {
    #[prop_or_default]
    pub width: Option<&'static str>,
    #[prop_or_default]
    pub height: Option<&'static str>,
}

#[function_component(Spacer)]
pub fn spacer(props: &SpacerProps) -> Html {
    let css = get_inline_style(&[("width", props.width), ("height", props.height)]);

    html! {
        <div style={css} />
    }
}
