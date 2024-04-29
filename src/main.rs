use crate::app::App;
use log::Level;

mod app;
mod components;
mod hooks;
mod routes;
mod screens;
mod test;
mod utils;
mod data;

fn main() {
    console_log::init_with_level(Level::Debug).expect("failed to init console log");
    yew::Renderer::<App>::new().render();
}
