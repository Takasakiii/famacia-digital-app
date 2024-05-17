use stylist::yew::styled_component;
use yew::{AttrValue, Callback, classes, Classes, html, Html, Properties};

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
    #[prop_or_default]
    pub icon: Option<AttrValue>,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let icon_class = css!(
        r#"
            margin-right: 0.5rem;
            margin-bottom: 0.25rem;
        "#
    );
    
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
        <button class={classes} onclick={on_click_callback} >
            {
                if let Some(icon) = &props.icon {
                    html! {
                        <i class={classes!(icon, icon_class)}></i>
                    }
                } else {
                    html! { }
                }   
            }
            {props.text}
        </button>
    }
}
