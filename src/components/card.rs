use yew::{function_component, html, Html};
use crate::components::spacer::Spacer;

#[function_component(Card)]
pub fn card() -> Html {
    html! {
        <div class="card">
            <div class="card-content">
                <p class="title is-4">{"UBS Jardim Centro"}</p>
                <div class="content">
                    {"Exemplo de avenida, 34 - Jardim Centro"}
                    <br/>
                    {"CEP: 06500-000"}
                </div>
            </div>
        </div>
    }
}
