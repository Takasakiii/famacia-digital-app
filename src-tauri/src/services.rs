use reqwest::Response;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://projetointegradorvi.ddns.net:5000/api";

#[derive(Deserialize, Serialize, Clone)]
pub struct Pharmacy {
    #[serde(rename(deserialize = "ID"))]
    id: i8,
    #[serde(rename(deserialize = "NomeFarmacia"))]
    name: Option<String>,
    #[serde(rename(deserialize = "Logradouro"))]
    address: Option<String>,
    #[serde(rename(deserialize = "CEP"))]
    cep: Option<String>,
    #[serde(rename(deserialize = "Número"))]
    number: Option<String>,
    #[serde(rename(deserialize = "Bairro"))]
    neighborhood: Option<String>,
    #[serde(rename(deserialize = "Telefone"))]   
    phone: Option<i32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Medication {
    #[serde(rename(deserialize = "ID"))]
    id: i8,
    #[serde(rename(deserialize = "nomeMedicamento"))]
    name: Option<String>,
    #[serde(rename(deserialize = "principioAtivo"))]
    active_principle: Option<String>,
    #[serde(rename(deserialize = "formaAdm"))]
    administration_form: Option<String>,
    #[serde(rename(deserialize = "classeMed"))]
    medication_class: Option<i8>,
    #[serde(rename(deserialize = "classificacao"))]
    classification: Option<i8>,
    #[serde(rename(deserialize = "retencaoReceita"))]
    prescription_retention: Option<i8>,
    #[serde(rename(deserialize = "tipoMed"))]
    medication_type: Option<String>,
    #[serde(rename(deserialize = "apresentacao"))]
    presentation: Option<String>,
    #[serde(rename(deserialize = "concentracao"))]
    concentration: Option<String>,
    #[serde(rename(deserialize = "fracionavel"))]
    fractionable: Option<i8>,
    #[serde(rename(deserialize = "usoContinuo"))]
    continuous_use: Option<i8>,
    #[serde(rename(deserialize = "obs"))]
    observation: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Stock {
    #[serde(rename(deserialize = "ID"))]
    id: i8,
    #[serde(rename(deserialize = "farmacia"))]
    pharmacy: Option<i8>,
    #[serde(rename(deserialize = "medicamento"))]
    medication: Option<i8>,
    #[serde(rename(deserialize = "saldo"))]
    balance: Option<f32>,
    #[serde(rename(deserialize = "nomeMedicamento"))]
    medication_name: Option<String>,
    #[serde(rename(deserialize = "concentracao"))]
    concentration: Option<String>,
    #[serde(rename(deserialize = "nomeFarmacia"))]
    pharmacy_name: Option<String>,
}

pub async fn get_pharmacies() -> Vec<Pharmacy> {
    let req = reqwest::get(format!("{}/get_json_farmac/all", BASE_URL)).await;
    parse(req, "buscar farmácias").await
}

pub async fn get_medications() -> Vec<Medication> {
    let req = reqwest::get(format!("{}/get_json_medic/all", BASE_URL)).await;
    parse(req, "buscar medicamentos").await
}

pub async fn get_stock_by_pharmacy(pharmacy_id: i8) -> Vec<Stock> {
    let req = reqwest::get(format!("{}/get_json_est_farm/{}", BASE_URL, pharmacy_id)).await;
    parse(req, "buscar estoque").await
}

pub  async fn get_stock_by_medication(medication_name: &str) -> Vec<Stock> {
    let req = reqwest::get(format!("{}/get_json_est_medic/{}", BASE_URL, medication_name)).await;
    parse(req, "buscar estoque").await
}

async fn parse<T>(req: reqwest::Result<Response>, identifier: &str) -> Vec<T>
where
    T: for<'de> Deserialize<'de>
{
    println!("Realizando request '{}'...", identifier);
    match req {
        Ok(res) => {
            let body = res.text().await.expect("Erro ao converter resposta em texto");
            let data = serde_json::from_str(&body);
            
            match data {
                Ok(data) => data,
                Err(e) => {
                    println!("Erro ao {}: {}", identifier, e);
                    println!("Resposta: {}", body);
                    vec![]
                }
            }
        }
        Err(e) => {
            println!("Erro ao {}: {}", identifier, e);
            vec![]
        }
    }
}

