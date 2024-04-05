use stylist::yew::{styled_component, use_media_query};
use yew::{html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct ScreenCenterProps {
    pub children: Html,
}

#[styled_component(ScreenCenter)]
pub fn screen_center(props: &ScreenCenterProps) -> Html {
    let min_height = use_media_query("(min-height: 490px)");


    if min_height {
        let css = css!(
            r#"
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                display: flex;
                align-items: center;
                justify-content: center;
            "#
        );
        html! {
        <div class={css}>
            {props.children.clone()}
        </div>
    }
    } else {
        props.children.clone()
    }
}
