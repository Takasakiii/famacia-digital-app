use std::rc::Rc;

use serde::Deserialize;
use yew::{AttrValue, hook, use_context, use_effect_with, use_state};

use crate::app::MedicationContext;
use crate::utils::call_tauri;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Medication {
    pub id: i8,
    pub name: Option<String>,
    pub active_principle: Option<String>,
    pub administration_form: Option<String>,
    pub medication_class: Option<i8>,
    pub classification: Option<i8>,
    pub prescription_retention: Option<i8>,
    pub medication_type: Option<String>,
    pub presentation: Option<String>,
    pub concentration: Option<String>,
    pub fractionable: Option<i8>,
    pub continuous_use: Option<i8>,
    pub observation: Option<String>,
}

#[hook]
pub fn use_medications() -> Option<Rc<Vec<Medication>>> {
    let medications = use_state(|| None);
    {
        let medications = medications.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let result: Vec<Medication> =
                    call_tauri("get_medications", &()).await;
                medications.set(Some(Rc::new(result)));
            });
        });
    }
    
    (*medications).clone()
}

#[hook]
pub fn use_medication_search(search: Option<AttrValue>) -> Option<Rc<Vec<Medication>>> {
    let medications = use_context::<MedicationContext>().unwrap()?;

    if let Some(search) = search {
        let result = medications.iter().filter(|medication| {
            let search = search.as_str().to_lowercase();
            medication.name.as_ref().map(|name| name.to_lowercase().contains(&search)).unwrap_or(false)
        }).cloned().collect::<Vec<Medication>>();
        Some(Rc::new(result))
    } else {
        Some(medications.clone())
    }
}

#[hook]
pub fn use_medication(id: i8) -> Option<Medication> {
    let medications = use_context::<MedicationContext>()??;
    medications.iter().find(|medication| medication.id == id).cloned()
}