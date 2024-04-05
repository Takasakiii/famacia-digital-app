use crate::screens::login::LoginScreen;
use crate::test::Test;
use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Login,
    #[at("/pharmacies")]
    Pharmacies,
    #[at("/medicines")]
    Medicines,
    #[at("/test")]
    Test,
}

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Login => html!(<LoginScreen />),
        Routes::Test => html!(<Test />),
        Routes::Pharmacies => html!(<Test />),
        Routes::Medicines => html!(<Test />),
    }
}

#[function_component(Router)]
pub fn route() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Routes> render={switch} />
        </BrowserRouter>
    }
}
