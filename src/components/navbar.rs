use stylist::css;
use stylist::yew::styled_component;
use yew::{html, Html};

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let back_button_css = css!(
        r#"
            width: 2rem;
            display: flex;
            justify-content: center;
        "#
    );

    html! {
        <nav class="navbar is-info is-fixed-top" role="navigation">
            <div class="navbar-brand">
                // <a class="navbar-item">
                //     <div class={back_button_css}>
                //         <i class="fa-solid fa-chevron-left"></i>
                //     </div>
                // </a>
                <a class="navbar-item" href="#">
                    <img src="/imgs/logo.png" alt="Logo da prefeitura de Vargem Grande Paulista" />
                </a>
            </div>
        </nav>
    }
}
