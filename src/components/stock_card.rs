use yew::{Callback, function_component, html, Html, Properties};
use yew_router::prelude::use_navigator;

use crate::components::card::Card;
use crate::hooks::stock::Stock;
use crate::routes::Routes;

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
    
    let (title, route) = match display_type {
        DisplayType::Pharmacy => (&stock.pharmacy_name, 
                                  Routes::Pharmacy { id: stock.pharmacy.unwrap_or(0) }),
        DisplayType::Medication => (&stock.medication_name, 
                                    Routes::Medication { id: stock.medication.unwrap_or(0) }),
    };

    let onclick = {
        let navigator = use_navigator().unwrap();
        Callback::from(move |_| {
            navigator.push(&route);
        })
    };
    
    html! {
        <Card {onclick} title={ title.clone().unwrap_or_else(String::new) }>
            <p>
                <b>{"Concentração: "}</b>
                { stock.concentration.as_ref().unwrap_or(&"Não informado".to_owned()) }
            </p>
            <p>
                <b>{"Estoque: "}</b>
                { stock.balance.unwrap_or(0.0) }
            </p>
        </Card>
    }
}