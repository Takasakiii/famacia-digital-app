use crate::components::navbar::Navbar;
use crate::screens::login::LoginScreen;
use crate::screens::pharmacies::Pharmacies;
use crate::test::Test;
use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Login,
    #[at("/test")]
    Test,
    #[at("/logged/*")]
    LoggedRouted,
}

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Login => html!(<LoginScreen />),
        Routes::Test => html!(<Test />),
        Routes::LoggedRouted => html! {
            <>
                <Navbar />
                <Switch<LoggedRoutes> render={switch_logged} />
            </>
        },
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum LoggedRoutes {
    #[at("/logged/pharmacies")]
    Pharmacies,
    #[at("/logged/medicines")]
    Medicines,
}

fn switch_logged(logged_routes: LoggedRoutes) -> Html {
    match logged_routes {
        LoggedRoutes::Pharmacies => html!(<Pharmacies />),
        LoggedRoutes::Medicines => html!(<Test />),
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
