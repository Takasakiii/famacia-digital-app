use serde::{Deserialize, Serialize};
use yew::{AttrValue, hook, use_effect_with, use_state};
use crate::utils::call_tauri;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Pharmacy {
    pub id: i8,
    pub name: String,
    pub address: String,
    pub cep: String,
    pub number: i16,
    pub neighborhood: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetPharmaciesRequest {
    pub search: Option<String>,
}

#[derive(Deserialize)]
pub struct GetPharmaciesResponse {
    pub pharmacies: Vec<Pharmacy>,
}

#[derive(Deserialize, Serialize)]
pub struct GetPharmacyRequest {
    pub id: i8,
}

#[hook]
pub fn use_pharmacies(search: Option<AttrValue>) -> Vec<Pharmacy> {
    let pharmacies = use_state(Vec::<Pharmacy>::new);
    {
        let pharmacies = pharmacies.clone();

        use_effect_with(search.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let result: GetPharmaciesResponse =
                    call_tauri("get_pharmacies", &GetPharmaciesRequest {
                        search: search.map(|x| x.as_str().to_owned())
                    }).await;
                pharmacies.set(result.pharmacies);
            });
        });
    }

    pharmacies.to_vec()
}

#[hook]
pub fn use_pharmacy(id: i8) -> Option<Pharmacy> {
    let pharmacy = use_state(|| None::<Pharmacy>);
    {
        let pharmacy = pharmacy.clone();

        use_effect_with(id, move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let result =
                    call_tauri("get_pharmacy", &GetPharmacyRequest {
                        id
                    }).await;
                pharmacy.set(result);
            });
        });
    }

    (*pharmacy).clone()
}