use yew::hook;

#[derive(Clone, PartialEq)]
pub struct Medication {
    pub id: i8,
    pub name: &'static str,
    pub concentration: &'static str,
}

pub static MEDICATIONS: [Medication; 3] = [
    Medication {
        id: 1,
        name: "Dipirona",
        concentration: "500mg",
    },
    Medication {
        id: 2,
        name: "Paracetamol",
        concentration: "750mg",
    },
    Medication {
        id: 3,
        name: "Ibuprofeno",
        concentration: "400mg",
    },
];

#[hook]
pub fn use_medications() -> Vec<Medication> {
    MEDICATIONS.to_vec()
}

#[hook]
pub fn use_medication(id: i8) -> Option<Medication> {
    MEDICATIONS.iter().find(|medication| medication.id == id).cloned()
}