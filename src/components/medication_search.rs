use yew::{function_component, Html, html, use_state};
use crate::components::input::Input;
use crate::components::medication_card::MedicationCard;
use crate::hooks::medication::use_medications;

#[function_component(MedicationSearch)]
pub fn medication_search() -> Html {
    let search_input = use_state(String::new);

    let medicines = use_medications(Some((*search_input).clone().into()));


    html! {
        <>
            <Input input_type="text" placeholder="Pesquisar..." value_state={search_input} />
            {
                if !medicines.is_empty() {
                    medicines.into_iter().map(|medication| {
                        html! {
                            <MedicationCard medication={ medication } />
                        }
                    }).collect::<Html>()
                } else {
                    html! {
                        <p>{"Nenhuma farmácia encontrada"}</p>
                    }
                }
            }
        </>
    }
}