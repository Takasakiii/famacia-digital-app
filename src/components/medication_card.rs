use yew::{Callback, function_component, html, Html, Properties};
use yew_router::prelude::use_navigator;

use crate::components::card::Card;
use crate::hooks::medication::Medication;
use crate::routes::Routes;
use crate::utils::get_default;

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
        <Card {onclick} 
              title={ get_default(medication.name.clone()) } 
              icon="fas fa-pills fa-2x">
            <p>
                <b>{"Concentração: "}</b>
                { get_default(medication.concentration.clone()) }
            </p>
            <p>
                <b>{"Princípio ativo: "}</b>
                { get_default(medication.active_principle.clone()) }
            </p>
            if *expanded {
                <p>
                    <b>{"Receita retida: "}</b>
                    if medication.prescription_retention == Some(1) {
                        {"Sim"}
                    } else {
                        {"Não"}
                    }
                </p>
            }
        </Card>
    }
}