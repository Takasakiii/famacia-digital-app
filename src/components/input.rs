use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct InputProps {
    pub label: &'static str,
    pub input_type: &'static str,
    pub placeholder: &'static str,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    html! {
        <div class="field">
          <label class="label">{props.label}</label>
          <div class="control">
            <input class="input" type={props.input_type} placeholder={props.placeholder} />
          </div>
        </div>
    }
}
