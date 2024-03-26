use yew::{Callback, classes, function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub text: &'static str,
    #[prop_or_default]
    pub color: Option<&'static str>,
    #[prop_or_default]
    pub full_width: bool,
    pub on_click: Callback<()>
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let full_width = if props.full_width {
        Some("is-fullwidth")
    } else {
        None
    };
    let classes = classes!("button", full_width, props.color);
    
    let on_click_callback = {
        let callback = props.on_click.clone();
        Callback::from(move |_| {
            callback.emit(())
        })
    };

    html! {
        <button class={classes} onclick={on_click_callback} >{props.text}</button>
    }
}
