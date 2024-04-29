use stylist::yew::styled_component;
use yew::{Html, html, Properties};
use crate::components::screen_padding::ScreenPadding;
use crate::components::card::Card;
use crate::hooks::pharmacy::use_pharmacy;

#[derive(PartialEq, Properties)]
pub struct PharmacyViewProps {
    pub id: i8,
}

#[styled_component(PharmacyView)]
pub fn pharmacy(props: &PharmacyViewProps) -> Html {
    let pharmacy = use_pharmacy(props.id);

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
                <Card title={ pharmacy.name.clone() }>
                    <p>
                        <b>{"Endereço: "}</b>
                        {format!("{}, {} - {}", &pharmacy.address, &pharmacy.number,
                            &pharmacy.neighborhood)}
                    </p>
                    <p>
                        <b>{"CEP: "}</b>
                        { &pharmacy.cep }
                    </p>
                </Card>
                <iframe class={ iframe_css } src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d14629.527727563647!2d-46.63773851598419!3d-23.55472254584241!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x94ce59a647219c9d%3A0x4af244dffee6ac6!2sDrogaria%20S%C3%A3o%20Paulo!5e0!3m2!1spt-BR!2sbr!4v1714369751096!5m2!1spt-BR!2sbr" allowfullscreen={true} loading="lazy" referrerpolicy="no-referrer-when-downgrade"></iframe>
            } else {
                <Card title="Carregando..." />
            }
        </ScreenPadding>
    }
}