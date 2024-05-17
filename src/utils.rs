use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::{AttrValue, Callback, TargetCast, UseStateHandle};

pub fn get_inline_style(css: &[(&str, Option<&str>)]) -> String {
    css.iter()
        .filter(|(_, v)| v.is_some())
        .map(|(k, v)| format!("{}: {}", k, v.unwrap()))
        .collect::<Vec<_>>()
        .join(";")
}

pub fn get_html_change_value(state: &UseStateHandle<String>) -> Callback<yew::prelude::InputEvent> {
    let state_clone = state.clone();
    Callback::from(move |e: yew::prelude::InputEvent| {
        let input = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = input {
            let value = input.value();
            state_clone.set(value);
        }
    })
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn call_tauri<T, J>(cmd: &str, args: &T) -> J
where
    T: serde::ser::Serialize,
    J: serde::de::DeserializeOwned,
{
    let args = to_value(args).unwrap();
    let response = invoke(cmd, args).await;
    from_value(response).unwrap()
}

pub fn format_cep(cep: AttrValue) -> String {
    if cep.len() == 8 {
        format!("{}-{}", &cep[0..5], &cep[5..8])
    } else {
        cep.to_string()
    }
}