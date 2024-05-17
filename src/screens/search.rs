use yew::{Callback, classes, function_component, Html, html, use_state};

use crate::components::button::Button;
use crate::components::medication_search::MedicationSearch;
use crate::components::pharmacy_search::PharmacySearch;
use crate::components::screen_padding::ScreenPadding;

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
    let search_type = use_state(|| SearchType::Pharmacy);

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
                    <Button text="Farmácia" 
                            class={classes!("is-fullwidth")}
                            color={ get_type_color(SearchType::Pharmacy, &search_type) }
                            on_click={ pharmacy_click }
                            icon="fas fa-house-chimney-medical" />
                </div>
                <div class="cell">
                    <Button text="Medicamento" class={classes!("is-fullwidth")}
                        color={ get_type_color(SearchType::Medicine, &search_type) }
                        on_click={ medicine_click }
                        icon="fas fa-prescription-bottle-medical" />
                </div>
            </div>
            {
                match *search_type {
                    SearchType::Pharmacy => {
                        html! {
                            <PharmacySearch />
                        }
                    },
                    SearchType::Medicine => {
                        html! {
                            <MedicationSearch />
                        }
                    }
                }
            }
        </ScreenPadding>
    }
}