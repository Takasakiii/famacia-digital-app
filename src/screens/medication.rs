use yew::{function_component, Html, html, Properties};
use crate::components::card::Card;

use crate::components::loading::Loading;
use crate::components::medication_card::MedicationCard;
use crate::components::screen_padding::ScreenPadding;
use crate::components::stock_card::{DisplayType, StockCard};
use crate::hooks::medication::use_medication;
use crate::hooks::stock::use_medication_stock;
use crate::utils::get_default;

#[derive(PartialEq, Properties)]
pub struct MedicationViewProps {
    pub id: i8,
}

#[function_component(MedicationView)]
pub fn medication(props: &MedicationViewProps) -> Html {
    let medication = use_medication(props.id);
    let name =
        medication.as_ref().map(|medication| medication.name.as_ref().map(|name| name.clone().into())).flatten();
    
    let stock = use_medication_stock(name);

    html! {
        <ScreenPadding>
            if let Some(medication) = medication {
                <MedicationCard medication={ medication.clone() } expanded={true} />
                <Card title="Informações" icon="fas fa-info fa-2x">
                    <p class="has-text-justified">
                        {"O medicamento "} <strong>{ get_default(medication.name) }</strong>
                        {" é do tipo "} <strong>{ get_default(medication.medication_type) }</strong> 
                        {", sendo apresentado por meio de "} <strong>{ get_default(medication.presentation) }</strong> 
                        {", com uma concentração de "} <strong>{ get_default(medication.concentration) }</strong> {". "}
                        {"Ele é administrado por via "} <strong>{ get_default(medication.administration_form) }</strong> 
                        {", "} <strong>{ if medication.continuous_use == Some(1) { "deve" } else { "não deve" } }</strong> {" ser usado continuamente "}
                        {"e "} <strong>{ if medication.fractionable == Some(1) { "pode" } else { "não pode" } }</strong> {" ser fracionado."}
                    </p>
                </Card>
                <h4 class="title is-4 my-5">{"Farmácias com estoque"}</h4>
                {
                    if let Some(stock) = stock {
                        if stock.is_empty() {
                            html! {
                                <p>{"Nenhuma farmácia encontrada"}</p>
                            }
                        } else {
                            stock.iter().map(|stock| {
                                html! {
                                    <StockCard stock={ stock.clone() }
                                        display_type={ DisplayType::Pharmacy } />
                                }
                            }).collect::<Html>()
                        }   
                    } else {
                        html! {
                            <Loading />
                        }
                    }
                 }
            } else {
                <Loading />
            }
        </ScreenPadding>
    }
}