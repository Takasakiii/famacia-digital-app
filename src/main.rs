use crate::app::App;

mod app;
mod components;
mod routes;
mod screens;
mod test;
mod utils;

fn main() {
    yew::Renderer::<App>::new().render();
}
