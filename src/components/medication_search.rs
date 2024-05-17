use yew::{function_component, Html, html, use_state};

use crate::components::input::Input;
use crate::components::medication_card::MedicationCard;
use crate::hooks::medication::use_medication_search;

#[function_component(MedicationSearch)]
pub fn medication_search() -> Html {
    let search_input = use_state(String::new);

    let medicines = use_medication_search(Some((*search_input).clone().into()));


    html! {
        <>
            <Input input_type="text" placeholder="Pesquisar..." value_state={search_input} />
            {
                if let Some(medicines) = medicines {
                    if !medicines.is_empty() {
                        medicines.iter().map(|medication| {
                            html! {
                                <MedicationCard medication={ medication.clone() } />
                            }
                        }).collect::<Html>()
                    } else {
                        html! {
                            <p>{"Nenhuma farmácia encontrada"}</p>
                        }
                    }
                } else {
                    html! {
                        <p>{"Carregando..."}</p>
                    }
                }
            }
        </>
    }
}