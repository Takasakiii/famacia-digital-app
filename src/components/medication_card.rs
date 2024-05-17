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
    
    let medication = medication.clone();

    html! {
        <Card {onclick} 
              title={ medication.name.unwrap_or_else(String::new) } 
              icon="fas fa-pills fa-2x">
            <p>
                <b>{"Concentração: "}</b>
                { medication.concentration.unwrap_or_else(|| "Não informado".to_owned()) }
            </p>
            <p>
                <b>{"Princípio ativo: "}</b>
                { medication.active_principle.unwrap_or_else(|| "Não informado".to_owned()) }
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