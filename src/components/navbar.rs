use crate::components::navbar_button::NavbarButton;
use crate::routes::Routes;
use stylist::style;
use yew::{classes, function_component, html, Html};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let justify = style!("justify-content: center;")
        .unwrap()
        .get_class_name()
        .to_owned();
    let classes = classes!("navbar-brand", justify);

    html! {
        <nav class="navbar is-link is-fixed-top" role="navigation">
            <div class={classes}>
                <NavbarButton label="Farmacias" route={Routes::Pharmacies} />
                <NavbarButton label="Remedios" route={Routes::Medicines} />
            </div>
        </nav>
    }
}
