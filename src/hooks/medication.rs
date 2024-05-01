use yew::{AttrValue, hook};

#[derive(Clone, PartialEq)]
pub struct Medication {
    pub id: i8,
    pub name: &'static str,
    pub concentration: &'static str,
    pub class: &'static str,
    pub presentation: &'static str,
    pub prescription_needed: bool,
}

pub static MEDICATIONS: [Medication; 4] = [
    Medication {
        id: 1,
        name: "Dipirona",
        concentration: "500mg",
        class: "Analgésico",
        presentation: "Comprimido",
        prescription_needed: false,
    },
    Medication {
        id: 2,
        name: "Paracetamol",
        concentration: "750mg",
        class: "Analgésico",
        presentation: "Comprimido",
        prescription_needed: false,
    },
    Medication {
        id: 3,
        name: "Ibuprofeno",
        concentration: "400mg",
        class: "Anti-inflamatório",
        presentation: "Líquido",
        prescription_needed: false,
    },
    Medication {
        id: 4,
        name: "Ritalina",
        concentration: "10mg",
        class: "Estimulante",
        presentation: "Comprimido",
        prescription_needed: true,
    }
];

#[hook]
pub fn use_medications(search: Option<AttrValue>) -> Vec<Medication> {
    let search = search.map(|search| search.to_string());
    if let Some(search) = search {
        let search = search.to_lowercase();
        MEDICATIONS
            .iter()
            .filter(|medication| medication.name.to_lowercase().contains(&search))
            .cloned()
            .collect()
    } else {
        MEDICATIONS.iter().cloned().collect()
    }
}

#[hook]
pub fn use_medication(id: i8) -> Option<Medication> {
    MEDICATIONS.iter().find(|medication| medication.id == id).cloned()
}