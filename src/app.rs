use yew::{function_component, Html, html};

use crate::routes::Router;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Router />
    }
}