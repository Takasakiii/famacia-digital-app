use yew::{classes, function_component, html, use_memo, Html, Properties};
use yew_router::hooks::use_location;
use yew_router::prelude::Link;
use yew_router::Routable;

use crate::routes::Routes;

#[derive(PartialEq, Properties)]
pub struct NavbarButtonProps {
    pub label: &'static str,
    pub route: Routes,
}

#[function_component(NavbarButton)]
pub fn navbar_button(props: &NavbarButtonProps) -> Html {
    let path = use_location().unwrap().path().to_owned();
    let is_active = use_memo((path, props.route.clone()), |(current, route)| {
        current.contains(route.to_path().as_str())
    });

    let normal_class = if !*is_active {
        Some("has-text-grey-light")
    } else {
        None
    };

    html! {
        <Link<Routes> classes={classes!("navbar-item", normal_class)} to={props.route.clone()}>{props.label}</Link<Routes>>
    }
}
