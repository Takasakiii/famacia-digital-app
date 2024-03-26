use crate::app::App;

mod app;
mod test;
mod routes;

fn main() {
    yew::Renderer::<App>::new().render();
}
