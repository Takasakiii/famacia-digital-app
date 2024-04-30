use stylist::yew::styled_component;
use yew::{Html, html, Properties};
use crate::components::screen_padding::ScreenPadding;
use crate::components::card::Card;
use crate::components::medication_card::MedicationCard;
use crate::components::pharmacy_card::PharmacyCard;
use crate::hooks::medication::use_medication;
use crate::hooks::pharmacy::use_pharmacies;

#[derive(PartialEq, Properties)]
pub struct MedicationViewProps {
    pub id: i8,
}

#[styled_component(MedicationView)]
pub fn medication(props: &MedicationViewProps) -> Html {
    let medication = use_medication(props.id);
    let pharmacies = use_pharmacies(None);

    html! {
        <ScreenPadding>
            if let Some(medication) = medication {
                <MedicationCard medication={ medication } />

                <h4 class="title is-4 my-5">{"Disponível em:"}</h4>
                {
                    pharmacies.into_iter().map(|pharmacy| {
                        html! {
                            <PharmacyCard pharmacy={ pharmacy } />
                        }
                    }).collect::<Html>()
                }
            } else {
                <Card title="Carregando..." />
            }
        </ScreenPadding>
    }
}