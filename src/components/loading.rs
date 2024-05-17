use yew::{function_component, html, Html};

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="has-text-centered my-6">
            <i class="fas fa-spinner fa-spin-pulse fa-4x" />
        </div>
    }
}
