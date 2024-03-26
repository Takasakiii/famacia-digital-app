use yew::{function_component, Html, html};

#[function_component(Test)]
pub fn test() -> Html {
    html! {
        <button class="button">{"Button"}</button>
    }
}