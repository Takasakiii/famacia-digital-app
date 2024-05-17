use stylist::yew::styled_component;
use yew::{html, Html};
use yew_router::prelude::{Link, use_route};
use crate::routes::Routes;

#[styled_component(Navbar)]
pub fn navbar() -> Html {
    let back_button_css = css!(
        r#"
            width: 2rem;
            display: flex;
            justify-content: center;
        "#
    );

    let route = use_route::<Routes>();

    html! {
        <nav class="navbar is-info is-fixed-top" role="navigation">
            <div class="navbar-brand">
                {
                    if let Some(route) = route {
                        if route != Routes::Search {
                            html! {
                                <Link<Routes> classes="navbar-item" to={Routes::Search}>
                                    <div class={back_button_css}>
                                        <i class="fa-solid fa-chevron-left"></i>
                                    </div>
                                </Link<Routes>>
                            }
                        }
                        else {
                            html! {}
                        }
                    } else {
                        html! {}
                    }
                }


                <a class="navbar-item" href="#">
                    <img src="/imgs/logo.png" alt="Logo da prefeitura de Vargem Grande Paulista" />
                </a>
            </div>
        </nav>
    }
}
