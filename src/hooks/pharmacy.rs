use serde::{Deserialize, Serialize};
use yew::{hook, use_effect, use_state};
use crate::utils::call_tauri;

#[derive(Deserialize, Clone)]
pub struct Pharmacy {
    pub id: i8,
    pub name: String,
    pub address: String,
    pub cep: String,
    pub number: i16,
    pub neighborhood: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetPharmaciesRequest<'a> {
    pub search: Option<&'a str>,
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
pub fn use_pharmacies(search: String) -> Vec<Pharmacy> {
    let pharmacies = use_state(Vec::<Pharmacy>::new);
    {
        let pharmacies = pharmacies.clone();

        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let result: GetPharmaciesResponse =
                    call_tauri("get_pharmacies", &GetPharmaciesRequest {
                        search: Some(search.as_str())
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

        use_effect(move || {
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