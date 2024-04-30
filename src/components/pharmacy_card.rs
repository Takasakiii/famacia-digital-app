use yew::{Callback, function_component, html, Html, Properties};
use yew_router::prelude::use_navigator;
use crate::components::card::Card;
use crate::hooks::pharmacy::Pharmacy;
use crate::routes::Routes;

#[derive(PartialEq, Properties)]
pub struct PharmacyCardProps {
    pub pharmacy: Pharmacy
}

#[function_component(PharmacyCard)]
pub fn pharmacy_card(props: &PharmacyCardProps) -> Html {
    let PharmacyCardProps { pharmacy } = props;
    let id = pharmacy.id;

    let onclick = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            navigator.push(&Routes::Pharmacy { id });
        })
    };
    html! {
         <Card {onclick} title={ pharmacy.name.clone() }>
            <p>{ format!("{}, {} - {}",
                    &pharmacy.address,
                    &pharmacy.number,
                    &pharmacy.neighborhood) }</p>
            <p>{ &pharmacy.cep }</p>
        </Card>
    }
}