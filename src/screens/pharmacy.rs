use stylist::yew::styled_component;
use yew::{Html, html, Properties};

use crate::components::card::Card;
use crate::components::screen_padding::ScreenPadding;
use crate::components::stock_card::{DisplayType, StockCard};
use crate::hooks::pharmacy::use_pharmacy;
use crate::hooks::stock::use_pharmacy_stock;

#[derive(PartialEq, Properties)]
pub struct PharmacyViewProps {
    pub id: i8,
}

#[styled_component(PharmacyView)]
pub fn pharmacy(props: &PharmacyViewProps) -> Html {
    let pharmacy = use_pharmacy(props.id);
    let stock = use_pharmacy_stock(props.id);

    let iframe_css = css!(
        r#"
            width: 100%;
            aspect-ratio: 16 / 9;
            border: 0;
            border-radius: 0.5rem;
        "#
    );

    html! {
        <ScreenPadding>
            if let Some(pharmacy) = pharmacy {
                <Card title={ pharmacy.name.unwrap_or_else(String::new) }>
                    <p>
                        <b>{"Endereço: "}</b>
                        {format!("{}, {} - {}", &pharmacy.address.unwrap_or_else(String::new),
                            &pharmacy.number.unwrap_or_else(String::new),
                            &pharmacy.neighborhood.unwrap_or_else(String::new))}
                    </p>
                    <p>
                        <b>{"CEP: "}</b>
                        { &pharmacy.cep.unwrap_or_else(String::new) }
                    </p>
                </Card>
                <iframe class={ iframe_css } src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d14629.527727563647!2d-46.63773851598419!3d-23.55472254584241!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x94ce59a647219c9d%3A0x4af244dffee6ac6!2sDrogaria%20S%C3%A3o%20Paulo!5e0!3m2!1spt-BR!2sbr!4v1714369751096!5m2!1spt-BR!2sbr" allowfullscreen={true} loading="lazy" referrerpolicy="no-referrer-when-downgrade"></iframe>
                <h4 class="title is-4 my-5">{"Medicamentos disponíveis"}</h4>
                
                {
                    if let Some(stock) = stock {
                        stock.iter().map(|stock| {
                            html! {
                                <StockCard stock={ stock.clone() }
                                    display_type={ DisplayType::Medication } />
                            }
                        }).collect::<Html>()
                    } else {
                        html! {
                            <Card title="Carregando..." />
                        }
                    }
                }
                
            } else {
                <Card title="Carregando..." />
            }
        </ScreenPadding>
    }
}