use yew::{Callback, function_component, Html, html, use_state};
use yew_router::prelude::use_navigator;
use crate::components::card::Card;
use crate::components::input::Input;
use crate::hooks::pharmacy::use_pharmacies;
use crate::routes::Routes;

#[function_component(PharmacySearch)]
pub fn medication_search() -> Html {
    let search_input = use_state(String::new);

    let pharmacies = use_pharmacies(Some((*search_input).clone().into()));

    let navigator = use_navigator().unwrap();
    let on_click = Callback::from(move |id: i8| {
        navigator.push(&Routes::Pharmacy { id });
    });


    html! {
        <>
            <Input input_type="text" placeholder="Pesquisar..." value_state={search_input} />
            {
                if !pharmacies.is_empty() {
                    pharmacies.iter().map(|pharmacy| {
                        let id = pharmacy.id;
                        let cb = on_click.clone();
                        let card_click = Callback::from(move |_| {
                            cb.emit(id);
                        });

                        html! {
                            <Card onclick={ &card_click } title={ pharmacy.name.clone() }>
                                <p>{ format!("{}, {} - {}",
                                        &pharmacy.address,
                                        &pharmacy.number,
                                        &pharmacy.neighborhood) }</p>
                                <p>{ &pharmacy.cep }</p>
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