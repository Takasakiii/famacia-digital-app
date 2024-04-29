use yew::{Callback, classes, function_component, Html, html, use_effect_with, use_state};
use yew_router::prelude::use_navigator;
use crate::components::button::Button;
use crate::components::screen_padding::ScreenPadding;
use crate::components::card::Card;
use crate::components::input::Input;
use crate::data::Pharmacy;
use crate::routes::Routes;
use crate::utils::{GetPharmaciesRequest, GetPharmaciesResponse};

#[function_component(SearchView)]
pub fn search_view() -> Html {
    let search_input = use_state(String::new);
    let pharmacies = use_state(Vec::<Pharmacy>::new);
    {
        let search_input = search_input.clone();
        let pharmacies = pharmacies.clone();

        use_effect_with(search_input.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let result: GetPharmaciesResponse =
                    crate::utils::call_tauri("get_pharmacies", &GetPharmaciesRequest {
                        search: Some(&search_input)
                    }).await;
                pharmacies.set(result.pharmacies);
            });
        });
    }

    let navigator = use_navigator().unwrap();
    let on_click = Callback::from(move |id: i8| {
        navigator.push(&Routes::Pharmacy { id });
    });

    html! {
        <ScreenPadding>
            <label class="label">{"Pesquisar por:"}</label>
            <div class="grid">
                <div class="cell">
                    <Button text="Farmacia" class={classes!("is-fullwidth")} color="is-link" />
                </div>
                <div class="cell">
                    <Button text="Medicamento" class={classes!("is-fullwidth")} />
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