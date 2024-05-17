use std::rc::Rc;

use serde::Deserialize;
use yew::{AttrValue, hook, use_context, use_effect_with, use_state};

use crate::app::PharmacyContext;
use crate::utils::call_tauri;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Pharmacy {
    pub id: i8,
    pub name: Option<String>,
    pub address: Option<String>,
    pub cep: Option<String>,
    pub number: Option<String>,
    pub neighborhood: Option<String>,
    pub phone: Option<i32>,
}

#[hook]
pub fn use_pharmacies() -> Option<Rc<Vec<Pharmacy>>> {
    let pharmacies = use_state(|| None);
    {
        let pharmacies = pharmacies.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let result: Vec<Pharmacy> =
                    call_tauri("get_pharmacies", &()).await;
                pharmacies.set(Some(Rc::new(result)));
            });
        });
    }

    (*pharmacies).clone()
}

#[hook]
pub fn use_pharmacy(id: i8) -> Option<Pharmacy> {
    let pharmacies = use_context::<PharmacyContext>()??;
    pharmacies.iter().find(|pharmacy| pharmacy.id == id).cloned()
}

#[hook]
pub fn use_pharmacy_search(search: Option<AttrValue>) -> Option<Rc<Vec<Pharmacy>>> {
    let pharmacies = use_context::<PharmacyContext>().unwrap()?;

    if let Some(search) = search {
        let result = pharmacies.iter().filter(|pharmacy| {
            let search = search.as_str().to_lowercase();
            pharmacy.name.as_ref().map(|name| name.to_lowercase().contains(&search)).unwrap_or(false)
        }).cloned().collect();
        Some(Rc::new(result))
    } else {
        Some(pharmacies.clone())
    }
}