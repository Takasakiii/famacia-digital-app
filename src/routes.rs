use yew::{function_component, Html, html};
use yew_router::{BrowserRouter, Routable, Switch};
use crate::test::Test;

#[derive(Clone, Routable, PartialEq)]
enum Routes {
    #[at("/")]
    Login,
    #[at("/test")]
    Test
}

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Login => html!(<h1>{"Login"}</h1>),
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