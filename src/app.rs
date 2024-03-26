use yew::{function_component, html, Html};

use crate::routes::Router;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Router />
    }
}
