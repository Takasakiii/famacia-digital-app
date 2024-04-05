use stylist::style;
use yew::{classes, function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct LogoProps {
    #[prop_or_default]
    pub center: bool,
    #[prop_or_default]
    pub is_title: bool,
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    let center = if props.center {
        Some("has-text-centered")
    } else {
        None
    };

    let title = if props.center {
        let display = style!("display: block;")
            .unwrap()
            .get_class_name()
            .to_owned();
        Some(["title".to_owned(), "is-fullwidth".to_owned(), display])
    } else {
        None
    };

    html! {
        <span class={classes!(title, "font-cookie", center)}>{"Farmacia Digital"}</span>
    }
}
