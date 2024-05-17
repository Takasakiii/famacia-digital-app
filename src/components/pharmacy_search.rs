use yew::{Callback, function_component, Html, html, use_state};
use yew_router::prelude::use_navigator;

use crate::components::card::Card;
use crate::components::input::Input;
use crate::components::loading::Loading;
use crate::hooks::pharmacy::use_pharmacy_search;
use crate::routes::Routes;
use crate::utils::format_cep;

#[function_component(PharmacySearch)]
pub fn medication_search() -> Html {
    let search_input = use_state(String::new);

    let pharmacies = use_pharmacy_search(Some((*search_input).clone().into()));

    let navigator = use_navigator().unwrap();
    let on_click = Callback::from(move |id: i8| {
        navigator.push(&Routes::Pharmacy { id });
    });


    html! {
        <>
            <Input input_type="text" placeholder="Pesquisar..." value_state={search_input} />
            {
                if let Some(pharmacies) = pharmacies {
                    if !pharmacies.is_empty() {
                        pharmacies.iter().map(|pharmacy| {
                            let id = pharmacy.id;
                            let cb = on_click.clone();
                            let card_click = Callback::from(move |_| {
                                cb.emit(id);
                            });
    
                            html! {
                                <Card onclick={ &card_click } 
                                    title={ pharmacy.name.clone().unwrap_or_else(String::new) }
                                    icon="fas fa-house-chimney-medical fa-2x">
                                    <p>{ format!("{}, {} - {}",
                                            &pharmacy.address.clone().unwrap_or_else(String::new),
                                            &pharmacy.number.clone().unwrap_or_else(String::new),
                                            &pharmacy.neighborhood.clone().unwrap_or_else(String::new)) }</p>
                                    <p>{ format_cep(pharmacy.cep.clone().unwrap_or_else(String::new).into()) }</p>
                                </Card>
                            }
                        }).collect::<Html>()
                    } else {
                        html! {
                            <p>{"Nenhuma farmácia encontrada"}</p>
                        }
                    }
                } else {
                    html! {
                        <Loading />
                    }
                }
            }
        </>
    }
}