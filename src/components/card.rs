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
    #[prop_or_default]
    pub icon: Option<AttrValue>,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    html! {
        <div class="card" onclick={ &props.onclick } >
            <div class="card-content">
                <div class="is-full is-flex is-justify-content-space-between mb-3">
                    <p class="title is-4 mr-2">{ &props.title }</p>
                    {
                        if let Some(icon) = &props.icon {
                            html! {
                                <i class={icon} />
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <div class="content">
                    { props.children.clone() }
                </div>
            </div>
        </div>
    }
}
