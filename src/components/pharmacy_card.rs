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
    
    let pharmacy = pharmacy.clone();
    
    html! {
         <Card {onclick} title={ pharmacy.name.unwrap_or_else(String::new) }>
            <p>{ format!("{}, {} - {}",
                    pharmacy.address.unwrap_or_else(String::new),
                    pharmacy.number.unwrap_or_else(String::new),
                    pharmacy.neighborhood.unwrap_or_else(String::new)) }</p>
            <p>{ &pharmacy.cep.unwrap_or_else(String::new) }</p>
        </Card>
    }
}