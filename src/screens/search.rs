use yew::{Callback, classes, function_component, Html, html, use_state};
use yew_router::prelude::use_navigator;

use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::input::Input;
use crate::components::screen_padding::ScreenPadding;
use crate::hooks::pharmacy::use_pharmacies;
use crate::routes::Routes;

#[derive(Copy, Clone, PartialEq)]
enum SearchType {
    Pharmacy,
    Medicine,
}

fn get_type_color(search_type: SearchType, current: &SearchType) -> &'static str {
    if search_type == *current {
        "is-link"
    } else {
        ""
    }
}

#[function_component(SearchView)]
pub fn search_view() -> Html {
    let search_input = use_state(String::new);
    let search_type = use_state(|| SearchType::Pharmacy);

    let pharmacies = use_pharmacies(Some((*search_input).clone().into()));

    let navigator = use_navigator().unwrap();
    let on_click = Callback::from(move |id: i8| {
        navigator.push(&Routes::Pharmacy { id });
    });

    let pharmacy_click = {
        let search_type = search_type.clone();
        Callback::from(move |_| {
            search_type.set(SearchType::Pharmacy);
        })
    };


    let medicine_click = {
        let search_type = search_type.clone();
        Callback::from(move |_| {
            search_type.set(SearchType::Medicine);
        })
    };

    html! {
        <ScreenPadding>
            <label class="label">{"Pesquisar por:"}</label>
            <div class="grid">
                <div class="cell">
                    <Button text="Farmacia" class={classes!("is-fullwidth")}
                        color={ get_type_color(SearchType::Pharmacy, &search_type) }
                        on_click={ pharmacy_click }/>
                </div>
                <div class="cell">
                    <Button text="Medicamento" class={classes!("is-fullwidth")}
                        color={ get_type_color(SearchType::Medicine, &search_type) }
                        on_click={ medicine_click }/>
                </div>
            </div>
            <Input input_type="text" placeholder="Pesquisar..." value_state={search_input} />
            {
                if pharmacies.is_empty() {
                    html! {
                        <h4>{"Nenhuma farmácia encontrada"}</h4>
                    }
                } else {
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
                }
            }
        </ScreenPadding>
    }
}