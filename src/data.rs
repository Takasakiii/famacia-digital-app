use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pharmacy {
    pub id: i8,
    pub name: String,
    pub address: String,
    pub cep: String,
    pub number: i16,
    pub neighborhood: String,
}
