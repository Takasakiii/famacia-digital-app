use yew::{Callback, function_component, html, Html, Properties};
use yew_router::prelude::use_navigator;
use crate::components::card::Card;
use crate::hooks::medication::Medication;
use crate::routes::Routes;

#[derive(PartialEq, Properties)]
pub struct MedicationCardProps {
    pub medication: Medication,
    #[prop_or_default]
    pub expanded: bool,
}

#[function_component(MedicationCard)]
pub fn medication_card(props: &MedicationCardProps) -> Html {
    let MedicationCardProps { medication, expanded } = props;
    let id = medication.id;

    let onclick = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            navigator.push(&Routes::Medication { id });
        })
    };

    html! {
        <Card {onclick} title={ medication.name }>
            <p>
                <b>{"Concentração: "}</b>
                { medication.concentration }
            </p>
            <p>
                <b>{"Apresentação: "}</b>
                { medication.presentation }
            </p>
            if *expanded {
                <p>
                    <b>{"Classe: "}</b>
                    { medication.class }
                </p>
                <p>
                    <b>{"Necessita de prescrição: "}</b>
                    { if medication.prescription_needed { "Sim" } else { "Não" } }
                </p>
            }
        </Card>
    }
}