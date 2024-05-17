// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod services;
use crate::services::{Medication, Pharmacy, Stock};

#[tauri::command]
async fn get_pharmacies() -> Vec<Pharmacy> {
    services::get_pharmacies().await
}

#[tauri::command]
async fn get_medications() -> Vec<Medication> {
    services::get_medications().await
}

#[tauri::command]
async fn get_pharmacy_stock(pharmacy_id: i8) -> Vec<Stock> {
    services::get_stock_by_pharmacy(pharmacy_id).await
}

#[tauri::command]
async fn get_medication_stock(medication_name: String) -> Vec<Stock> {
    services::get_stock_by_medication(medication_name.as_str()).await
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_pharmacies, get_medications, get_pharmacy_stock, get_medication_stock])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
