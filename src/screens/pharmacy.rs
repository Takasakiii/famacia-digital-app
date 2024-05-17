use stylist::yew::styled_component;
use url::form_urlencoded::byte_serialize;
use yew::{Html, html, Properties};

use crate::components::card::Card;
use crate::components::loading::Loading;
use crate::components::screen_padding::ScreenPadding;
use crate::components::stock_card::{DisplayType, StockCard};
use crate::hooks::pharmacy::use_pharmacy;
use crate::hooks::stock::use_pharmacy_stock;
use crate::utils::{format_cep, get_default};

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
    
    let cep = format_cep(pharmacy.as_ref().map(|x| x.cep.clone()).flatten().unwrap_or_else(String::new).into());
    
    let address = if let Some(pharmacy) = pharmacy.clone() {
        format!("{}, {} - {}", get_default(pharmacy.address),
                get_default(pharmacy.number),
                get_default(pharmacy.neighborhood))
    } else {
        String::new()
    };
    
    html! {
        <ScreenPadding>
            if let Some(pharmacy) = pharmacy {
                <Card title={ get_default(pharmacy.name) } 
                      icon="fas fa-house-chimney-medical fa-2x">
                    <p>
                        <b>{"Endereço: "}</b>
                        { &address }
                    </p>
                    <p>
                        <b>{"CEP: "}</b>
                        { &cep }
                    </p>
                </Card>
                    <iframe class={ iframe_css }
                            src={format!("https://maps.google.com/maps?q={}z=15&output=embed",
                                byte_serialize(format!("{} - {} - {}", address, "Vargem Grande Paulista", cep).as_bytes()).collect::<String>())}
                            allowfullscreen={true}
                            loading="lazy"
                            referrerpolicy="no-referrer-when-downgrade"></iframe>
            
                <h4 class="title is-4 my-5">{"Medicamentos disponíveis"}</h4>
                
                {
                    if let Some(stock) = stock {
                        if stock.is_empty() {
                            html! {
                                <p>{"Nenhum medicamento encontrado"}</p>
                            }
                        } else {
                            stock.iter().map(|stock| {
                                html! {
                                    <StockCard stock={ stock.clone() }
                                        display_type={ DisplayType::Medication } />
                                }
                            }).collect::<Html>()
                        }
                    } else {
                        html! {
                            <Loading />
                        }
                    }
                }
            } else {
                <Loading />
            }
        </ScreenPadding>
    }
}