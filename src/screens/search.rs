use yew::{classes, function_component, Html, html, use_state};
use crate::components::button::Button;
use crate::components::screen_padding::ScreenPadding;
use crate::components::card::Card;
use crate::components::input::Input;

#[function_component(SearchView)]
pub fn search_view() -> Html {
    let search_input = use_state(String::new);
    
    html! {
        <ScreenPadding>
            <label class="label">{"Pesquisar por:"}</label>
            <div class="grid">
                <div class="cell">
                    <Button text="Farmacia" class={classes!("is-fullwidth")} color="is-link" />
                </div>
                <div class="cell">
                    <Button text="Medicamento" class={classes!("is-fullwidth")} />
                </div>
            </div>
            <Input input_type="text" placeholder="Pesquisar..." value_state={search_input} />
            
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            <Card />
            
        </ScreenPadding>
    }
}