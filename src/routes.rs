use crate::components::navbar::Navbar;

use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};
use crate::components::spacer::Spacer;
use crate::screens::search::SearchView;

#[derive(Clone, Routable, PartialEq)]
pub enum Routes {
    #[at("/")]
    Search,
}

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Search => html!(<SearchView />),
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
