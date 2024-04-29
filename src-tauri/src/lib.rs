// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Pharmacy {
    id: i8,
    name: &'static str,
    address: &'static str,
    cep: &'static str,
    number: i16,
    neighborhood: &'static str,
}

#[derive(Serialize)]
pub struct GetPharmacyResponse {
    pharmacies: Vec<Pharmacy>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn login(email: &str, password: &str) -> bool {
    email == "user@email.com" && password == "password"
}

const PHARMACIES: &[Pharmacy] = &[
    Pharmacy {
        id: 1,
        name: "UBS Jardim Centro",
        address: "Exemplo de avenida",
        cep: "06500-000",
        number: 34,
        neighborhood: "Jardim Centro",
    },
    Pharmacy {
        id: 2,
        name: "Farmácia Pública",
        address: "Exemplo de avenida 2",
        cep: "06500-000",
        number: 40,
        neighborhood: "Jardim Centro",
    },
];

#[tauri::command]
fn get_pharmacies(search: Option<&str>) -> GetPharmacyResponse {
    let mut pharmacies = PHARMACIES.to_vec();

    if let Some(search) = search {
        let search = search.to_lowercase();
        pharmacies = pharmacies
            .into_iter()
            .filter(|pharmacy| pharmacy.name.to_lowercase().contains(&search))
            .collect();
    }

    GetPharmacyResponse {
        pharmacies
    }
}

#[tauri::command]
fn get_pharmacy(id: i8) -> Option<Pharmacy> {
    PHARMACIES.iter().find(|pharmacy| pharmacy.id == id).cloned()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, login, get_pharmacies, get_pharmacy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
