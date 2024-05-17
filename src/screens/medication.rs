use yew::{function_component, Html, html, Properties};

use crate::components::loading::Loading;
use crate::components::medication_card::MedicationCard;
use crate::components::screen_padding::ScreenPadding;
use crate::components::stock_card::{DisplayType, StockCard};
use crate::hooks::medication::use_medication;
use crate::hooks::stock::use_medication_stock;

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
                <MedicationCard medication={ medication } expanded={true} />
                {
                    if let Some(stock) = stock {
                        stock.iter().map(|stock| {
                            html! {
                                <StockCard stock={ stock.clone() }
                                    display_type={ DisplayType::Pharmacy } />
                            }
                        }).collect::<Html>()
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