use crate::components::navbar::Navbar;
use yew::{function_component, html, Html};

#[function_component(Test)]
pub fn test() -> Html {
    html! {
        <>
            <Navbar />
        </>

    }
}
