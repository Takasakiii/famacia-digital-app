use crate::components::navbar::Navbar;

use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};
use crate::components::spacer::Spacer;
use crate::screens::medication::MedicationView;
use crate::screens::pharmacy::PharmacyView;
use crate::screens::search::SearchView;

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Search,
    #[at("/pharmacies/:id")]
    Pharmacy { id: i8 },
    #[at("/medications/:id")]
    Medication { id: i8 },
}

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Search => html!(<SearchView />),
        Routes::Pharmacy { id } => html!(<PharmacyView {id} />),
        Routes::Medication { id } => html!(<MedicationView {id} />),
    }
}


#[function_component(Router)]
pub fn route() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <Spacer height="56px" />
            <Switch<Routes> render={switch} />
        </BrowserRouter>
    }
}
