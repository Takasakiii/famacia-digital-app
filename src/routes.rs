use crate::screens::login::LoginScreen;
use crate::test::Test;
use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

#[derive(Clone, Routable, PartialEq)]
enum Routes {
    #[at("/")]
    Login,
    #[at("/test")]
    Test,
}

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Login => html!(<LoginScreen />),
        Routes::Test => html!(<Test />),
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
