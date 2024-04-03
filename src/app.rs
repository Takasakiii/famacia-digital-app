use crate::components::toast::Toast;
use crate::hooks::toast_hook::{use_toast, ToastControls};
use yew::{function_component, html, ContextProvider, Html};

use crate::routes::Router;

#[function_component(App)]
pub fn app() -> Html {
    let toast = use_toast();

    html! {
        <ContextProvider<ToastControls> context={toast}>
            <Router />
            <Toast />
        </ContextProvider<ToastControls>>
    }
}
