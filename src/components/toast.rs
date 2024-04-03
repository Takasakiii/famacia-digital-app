use stylist::yew::styled_component;
use yew::{classes, html, use_context, Html};

use crate::hooks::toast_hook::ToastControls;

#[styled_component(Toast)]
pub fn toast() -> Html {
    let toast_context = use_context::<ToastControls>().expect("Could not find toast context");

    let current_toast = toast_context.current_toast;

    let css = css!(
        r#"
            position: fixed;
            left: 0;
            right: 0;
            bottom: 0;
            z-index: 1000;
        "#
    );

    html! {
        if let Some(current_toast) = current_toast {
            <div class={css}>
                <div class={classes!("notification", current_toast.color)}>
                    {current_toast.message}
                </div>
            </div>
        }
    }
}
