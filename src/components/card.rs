use yew::{function_component, Html, html};

#[function_component(Card)]
pub fn card() -> Html {
    html! {
        <div class="card">
            <div class="card-content">
                <div class="media-content">
                    <p class="title is-4">{"John Smith"}</p>
                    <p class="subtitle is-6">{"@johnsmith"}</p>
                </div>
            </div>
            <div class="content">
                {"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus nec"}
            </div>
        </div>
    }
}