use yew::{classes, function_component, html, Callback, Html, Properties, Classes};

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub text: &'static str,
    #[prop_or_default]
    pub color: Option<&'static str>,
    #[prop_or_default]
    pub full_width: bool,
    #[prop_or_default]
    pub on_click: Option<Callback<()>>,
    #[prop_or_default]
    pub class: Option<Classes>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let full_width = if props.full_width {
        Some("is-fullwidth")
    } else {
        None
    };
    let classes = classes!("button", full_width, props.color, &props.class);

    let on_click_callback = {
        let callback = props.on_click.clone();
        Callback::from(move |_| if let Some(callback) = &callback {
            callback.emit(());
        })
    };

    html! {
        <button class={classes} onclick={on_click_callback} >{props.text}</button>
    }
}
