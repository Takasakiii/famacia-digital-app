use yew::{Callback, function_component, html, Html, Properties};
use yew_router::prelude::use_navigator;
use crate::components::card::Card;
use crate::hooks::medication::Medication;
use crate::routes::Routes;

#[derive(PartialEq, Properties)]
pub struct MedicationCardProps {
    pub medication: Medication,
}

#[function_component(MedicationCard)]
pub fn medication_card(props: &MedicationCardProps) -> Html {
    let MedicationCardProps { medication } = props;
    let id = medication.id;

    let onclick = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            navigator.push(&Routes::Medication { id });
        })
    };

    html! {
        <Card {onclick} title={ medication.name }>
            <b>{"Concentração: "}</b>
            { medication.concentration }
        </Card>
    }
}