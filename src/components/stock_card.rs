use yew::{Callback, function_component, html, Html, Properties};
use yew_router::prelude::use_navigator;

use crate::components::card::Card;
use crate::hooks::stock::Stock;
use crate::routes::Routes;
use crate::utils::get_default;

#[derive(PartialEq, Properties)]
pub struct StockCardProps {
    pub stock: Stock,
    pub display_type: DisplayType,
}

#[derive(PartialEq)]
pub enum DisplayType {
    Pharmacy,
    Medication,
}

#[function_component(StockCard)]
pub fn stock_card(props: &StockCardProps) -> Html {
    let StockCardProps { stock, display_type } = props;
    
    let (title, route, icon) = match display_type {
        DisplayType::Pharmacy => (&stock.pharmacy_name, 
                                  Routes::Pharmacy { id: stock.pharmacy.unwrap_or(0) },
                                  "fas fa-house-chimney-medical fa-2x"),
        DisplayType::Medication => (&stock.medication_name, 
                                    Routes::Medication { id: stock.medication.unwrap_or(0) },
                                    "fas fa-pills fa-2x"),
    };

    let onclick = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            navigator.push(&route);
        })
    };
    
    html! {
        <Card {onclick} {icon} title={ get_default(title.clone()) }>
            <p>
                <b>{"Concentração: "}</b>
                { get_default(stock.concentration.clone()) }
            </p>
            <p>
                <b>{"Estoque: "}</b>
                { stock.balance.unwrap_or(0.0) }
                {" unidades"}
            </p>
        </Card>
    }
}