use web_sys::MouseEvent;
use yew::{AttrValue, Callback, function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct CardProps {
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="card" onclick={ &props.onclick } >
            <div class="card-content">
                <p class="title is-4">{ &props.title }</p>
                <div class="content">
                    { props.children.clone() }
                </div>
            </div>
        </div>
    }
}
