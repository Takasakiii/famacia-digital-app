use std::rc::Rc;

use serde::{Deserialize, Serialize};
use yew::{AttrValue, hook, use_effect_with, use_state};

use crate::utils::call_tauri;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Stock {
    pub id: i8,
    pub pharmacy: Option<i8>,
    pub medication: Option<i8>,
    pub balance: Option<f32>,
    pub medication_name: Option<String>,
    pub concentration: Option<String>,
    pub pharmacy_name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStockByPharmacyRequest {
    pub pharmacy_id: i8,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStockByMedicationRequest {
    pub medication_name: String,
}

#[hook]
pub fn use_pharmacy_stock(id: i8) -> Option<Rc<Vec<Stock>>> {
    println!("use_pharmacy_stock called!");
    let stock = use_state(|| None);
    {
        let stock = stock.clone();

        use_effect_with(id, move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let result: Vec<Stock> =
                    call_tauri("get_pharmacy_stock", &GetStockByPharmacyRequest {
                        pharmacy_id: id,
                    }).await;
                stock.set(Some(Rc::new(result)));
            });
        });
    }

    (*stock).clone()
}

#[hook]
pub fn use_medication_stock(name: Option<AttrValue>) -> Option<Rc<Vec<Stock>>> {
    println!("Hook: {:?}", name);
    let stock = use_state(|| None);
    {
        let stock = stock.clone();

        use_effect_with(name.clone(), move |_| {
            if let Some(name) = name {
                wasm_bindgen_futures::spawn_local(async move {
                    let result: Vec<Stock> =
                        call_tauri("get_medication_stock", &GetStockByMedicationRequest {
                            medication_name: name.to_string(),
                        }).await;
                    stock.set(Some(Rc::new(result)));
                });
            }
        });
    }

    (*stock).clone()
}
