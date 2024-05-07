use crate::utils::get_html_change_value;
use yew::{function_component, html, Html, Properties, UseStateHandle};

#[derive(PartialEq, Properties)]
pub struct InputProps {
    #[prop_or_default]
    pub label: Option<&'static str>,
    pub input_type: &'static str,
    pub placeholder: &'static str,
    pub value_state: UseStateHandle<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let input_callback = get_html_change_value(&props.value_state);

    html! {
        <div class="field">
            if let Some(label) = props.label {
                <label class="label">{label}</label>
            }   
            <div class="control">
                <input class="input" type={props.input_type} placeholder={props.placeholder} oninput={input_callback} value={(*props.value_state).clone()} />
            </div>
        </div>
    }
}
