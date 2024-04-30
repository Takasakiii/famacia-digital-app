use yew::{Callback, function_component, Html, html, use_state};
use yew_router::prelude::use_navigator;
use crate::components::card::Card;
use crate::components::input::Input;
use crate::hooks::medication::use_medications;
use crate::routes::Routes;

#[function_component(MedicationSearch)]
pub fn medication_search() -> Html {
    let search_input = use_state(String::new);

    let medicines = use_medications(Some((*search_input).clone().into()));

    let navigator = use_navigator().unwrap();
    let on_click = Callback::from(move |id: i8| {
        navigator.push(&Routes::Medication { id });
    });


    html! {
        <>
            <Input input_type="text" placeholder="Pesquisar..." value_state={search_input} />
            {
                if !medicines.is_empty() {
                    medicines.into_iter().map(|medication| {
                        let cb = on_click.clone();
                        let card_click = Callback::from(move |_| {
                            cb.emit(medication.id);
                        });

                        html! {
                            <Card title={ medication.name } onclick={ card_click }>
                                <p>{ &medication.name }</p>
                            </Card>
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