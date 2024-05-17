use std::rc::Rc;
use crate::components::toast::Toast;
use crate::hooks::toast_hook::{use_toast, ToastControls};
use yew::{function_component, html, ContextProvider, Html};
use crate::hooks::medication::{Medication, use_medications};
use crate::hooks::pharmacy::Pharmacy;
use crate::hooks::pharmacy::use_pharmacies;

use crate::routes::Router;

pub type PharmacyContext = Option<Rc<Vec<Pharmacy>>>;
pub  type MedicationContext = Option<Rc<Vec<Medication>>>;

#[function_component(App)]
pub fn app() -> Html {
    let toast = use_toast();
    let pharmacies = use_pharmacies();
    let medications = use_medications();
    
    html! {
        <ContextProvider<ToastControls> context={toast}>
            <ContextProvider<PharmacyContext> context={pharmacies}>
                <ContextProvider<MedicationContext> context={medications}>
                    <Router />
                    <Toast />
                </ContextProvider<MedicationContext>>
            </ContextProvider<PharmacyContext>>
        </ContextProvider<ToastControls>>
    }
}
