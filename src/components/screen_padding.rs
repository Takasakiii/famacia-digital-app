use stylist::yew::styled_component;
use yew::{html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ScreenPaddingProps {
    pub children: Html,
}

#[styled_component(ScreenPadding)]
pub fn screen_padding(props: &ScreenPaddingProps) -> Html {
    let css = css!(
        r#"
            width: 100%;
            height: fit-content;
            padding: 1rem;
        "#
    );

    html! {
        <div class={css}>
            {props.children.clone()}
        </div>
    }
}
